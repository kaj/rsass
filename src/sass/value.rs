use crate::css;
use crate::error::Error;
use crate::ordermap::OrderMap;
use crate::output::Format;
use crate::parser::SourcePos;
use crate::sass::{CallArgs, SassString};
use crate::value::{ListSeparator, Number, Numeric, Operator, Quotes, Rgba};
use crate::ScopeRef;
use num_traits::Zero;

/// A sass value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Value {
    /// A special kind of escape.  Only really used for !important.
    Bang(String),
    /// A call has a name and an argument (which may be multi).
    Call(SassString, CallArgs, SourcePos),
    /// A literal string value (quoted or not).
    Literal(SassString),
    /// A comma- or space separated list of values, with or without brackets.
    List(Vec<Value>, ListSeparator, bool),
    /// A Numeric value is a rational value with a Unit (which may be
    /// Unit::None) and flags.
    Numeric(Numeric),
    /// "(a/b) and a/b differs semantically.  Parens means the value
    /// should be evaluated numerically if possible, without parens /
    /// is not allways division.
    /// The boolean tells if the paren itself should be kept for output.
    Paren(Box<Value>, bool),
    /// A variable reference to be loaded when the value is evaluated.
    Variable(String),
    /// Both a numerical and original string representation,
    /// since case and length should be preserved (#AbC vs #aabbcc).
    Color(Rgba, Option<String>),
    /// The null value.
    Null,
    /// The true boolean value.
    True,
    /// The false boolean value.
    False,
    /// A binary operation, two operands and an operator.
    /// The boolean represents possible whitespace.
    BinOp(Box<Value>, bool, Operator, bool, Box<Value>),
    /// A unary operator and its operand.
    UnaryOp(Operator, Box<Value>),
    /// A map in sass source is just a list of key/value parirs.
    /// Actual map behaviour comes after evaluating it.
    Map(Vec<(Value, Value)>),
    /// The magic value "&", exanding to the current selectors.
    HereSelector,
    /// A unicode range for font selections. U+NN, U+N?, U+NN-MM.
    /// The string is the entire value, including the "U+" tag.
    UnicodeRange(String),
}

impl Value {
    /// Create a new scalar value.
    pub fn scalar(v: impl Into<Number>) -> Self {
        Value::Numeric(Numeric::scalar(v))
    }

    #[cfg(test)]
    pub fn black() -> Self {
        Value::Color(Rgba::from_rgb(0, 0, 0), Some("black".into()))
    }

    /// All values other than `False` and `Null` should be considered true.
    pub fn is_true(&self) -> bool {
        !matches!(self, Value::False | Value::Null)
    }

    /// Return true if this value is null.
    ///
    /// Note that an empty unquoted string and a list containing no
    /// non-null values is also considered null.
    pub fn is_null(&self) -> bool {
        match *self {
            Value::Null => true,
            Value::List(ref list, _, false) => {
                list.iter().all(|v| v.is_null())
            }
            _ => false,
        }
    }

    /// Evaluate this value in a given scope.
    pub fn evaluate(&self, scope: ScopeRef) -> Result<css::Value, Error> {
        self.do_evaluate(scope, false)
    }

    /// Evaluate this value to a [`css::Value`].
    pub fn do_evaluate(
        &self,
        scope: ScopeRef,
        arithmetic: bool,
    ) -> Result<css::Value, Error> {
        match *self {
            Value::Bang(ref s) => Ok(css::Value::Bang(s.clone())),
            Value::Literal(ref s) => {
                let (s, q) = s.evaluate(scope)?;
                if s.is_empty() && q == Quotes::None {
                    Ok(css::Value::Null)
                } else {
                    Ok(css::Value::Literal(s, q))
                }
            }
            Value::Paren(ref v, ref expl) => {
                let v = v.do_evaluate(scope, !expl)?;
                if !expl {
                    Ok(v)
                } else {
                    Ok(css::Value::Paren(Box::new(v)))
                }
            }
            Value::Color(ref rgba, ref name) => {
                Ok(css::Value::Color(rgba.clone().into(), name.clone()))
            }
            Value::Variable(ref name) => {
                Ok(scope.get(name)?.into_calculated())
            }
            Value::List(ref v, s, b) => {
                let items = v
                    .iter()
                    .map(|v| v.do_evaluate(scope.clone(), false))
                    .collect::<Result<Vec<_>, Error>>()?;
                Ok(css::Value::List(items, s, b))
            }
            Value::Call(ref name, ref args, ref pos) => {
                let args = args.evaluate(scope.clone(), true)?;
                if let Some(name) = name.single_raw() {
                    if let Some(f) = scope.get_function(&name.into()) {
                        f.call(scope.clone(), &args).map_err(|e| match e {
                            Error::BadArguments(msg, decl) => {
                                Error::BadCall(msg, pos.clone(), Some(decl))
                            }
                            e => Error::BadCall(
                                e.to_string(),
                                pos.clone(),
                                None,
                            ),
                        })
                    } else {
                        Ok(css::Value::Call(name.to_string(), args))
                    }
                } else {
                    let (name, _) = name.evaluate(scope)?;
                    Ok(css::Value::Call(name, args))
                }
            }
            Value::Numeric(ref num) => {
                Ok(css::Value::Numeric(num.clone(), arithmetic))
            }
            Value::Map(ref m) => {
                let items = m
                    .iter()
                    .map(|&(ref k, ref v)| {
                        Ok((
                            k.do_evaluate(scope.clone(), false)?,
                            v.do_evaluate(scope.clone(), false)?,
                        ))
                    })
                    .collect::<Result<OrderMap<_, _>, Error>>()?;
                Ok(css::Value::Map(items))
            }
            Value::Null => Ok(css::Value::Null),
            Value::True => Ok(css::Value::True),
            Value::False => Ok(css::Value::False),
            Value::BinOp(ref a, _, ref op, _, ref b) => {
                let (a, b) = {
                    let arithmetic = arithmetic | (*op != Operator::Div);
                    let aa = a.do_evaluate(scope.clone(), arithmetic)?;
                    let b = b.do_evaluate(
                        scope.clone(),
                        arithmetic || aa.is_calculated(),
                    )?;
                    if !arithmetic && b.is_calculated() && !aa.is_calculated()
                    {
                        (a.do_evaluate(scope, true)?, b)
                    } else {
                        (aa, b)
                    }
                };
                Ok(op.eval(a.clone(), b.clone()).unwrap_or_else(|| {
                    css::Value::BinOp(
                        Box::new(a),
                        false,
                        op.clone(),
                        false,
                        Box::new(b),
                    )
                }))
            }
            Value::UnaryOp(ref op, ref v) => {
                let value = v.do_evaluate(scope, true)?;
                match (op.clone(), value) {
                    (Operator::Not, css::Value::Numeric(v, _)) => {
                        Ok(v.value.is_zero().into())
                    }
                    (Operator::Not, css::Value::True) => {
                        Ok(css::Value::False)
                    }
                    (Operator::Not, css::Value::False) => {
                        Ok(css::Value::True)
                    }
                    (Operator::Minus, css::Value::Numeric(v, _)) => {
                        Ok(css::Value::Numeric(-&v, true))
                    }
                    (Operator::Plus, css::Value::Numeric(v, _)) => {
                        Ok(css::Value::Numeric(v, true))
                    }
                    (
                        Operator::Minus,
                        css::Value::Literal(s, Quotes::None),
                    ) => Ok(css::Value::Literal(
                        format!("-{}", s),
                        Quotes::None,
                    )),
                    (op, v) => Ok(css::Value::UnaryOp(op, Box::new(v))),
                }
            }
            Value::HereSelector => Ok(scope.get_selectors().to_value()),
            Value::UnicodeRange(ref s) => {
                Ok(css::Value::UnicodeRange(s.clone()))
            }
        }
    }

    /// Write a string representation of this value
    ///
    /// This does _not_ evaluate the value.
    pub fn inspect(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::fmt::Display;
        match *self {
            Value::Bang(ref s) => write!(out, "!{}", s),
            Value::Literal(ref s) => {
                write!(out, "{:?}", s)
            }
            Value::Paren(ref v, _expl) => {
                out.write_str("(")?;
                v.inspect(out)?;
                out.write_str(")")
            }
            Value::Color(ref rgba, ref name) => {
                if let Some(name) = name {
                    out.write_str(name)
                } else {
                    crate::value::Color::from(rgba.clone())
                        .format(Format::introspect())
                        .fmt(out)
                }
            }
            Value::Variable(ref name) => {
                write!(out, "${}", name)
            }
            Value::List(ref v, s, b) => {
                if b {
                    out.write_str("(")?;
                }
                if let Some((first, rest)) = v.split_first() {
                    first.inspect(out)?;
                    for i in rest {
                        out.write_str(if s == ListSeparator::Space {
                            " "
                        } else {
                            ", "
                        })?;
                        i.inspect(out)?;
                    }
                }
                if b {
                    out.write_str(")")?;
                }
                Ok(())
            }
            Value::Call(ref name, ref args, ref _pos) => {
                write!(out, "{}({:?})", name, args)
            }
            Value::Numeric(ref num) => {
                num.format(Format::introspect()).fmt(out)
            }
            Value::Map(ref m) => {
                out.write_str("(")?;
                if let Some(((k, v), rest)) = m.split_first() {
                    k.inspect(out)?;
                    out.write_str(": ")?;
                    v.inspect(out)?;
                    for (k, v) in rest {
                        out.write_str(", ")?;
                        k.inspect(out)?;
                        out.write_str(": ")?;
                        v.inspect(out)?;
                    }
                }
                out.write_str(")")
            }
            Value::Null => out.write_str("null"),
            Value::True => out.write_str("true"),
            Value::False => out.write_str("false"),
            Value::BinOp(ref a, _, ref op, _, ref b) => {
                a.inspect(out)?;
                op.fmt(out)?;
                b.inspect(out)
            }
            Value::UnaryOp(ref op, ref v) => {
                op.fmt(out)?;
                v.inspect(out)
            }
            Value::HereSelector => out.write_str("&"),
            Value::UnicodeRange(ref s) => s.fmt(out),
        }
    }
}
