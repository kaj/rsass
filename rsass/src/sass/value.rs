use super::{CallArgs, Function, Name, SassString};
use crate::error::ResultPos;
use crate::input::SourcePos;
use crate::output::Format;
use crate::value::{BadOp, ListSeparator, Number, Numeric, Operator, Rgba};
use crate::{css, Error, Invalid, ScopeRef};
use std::fmt::{self, Write};

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
    List(Vec<Value>, Option<ListSeparator>, bool),
    /// A Numeric value is a rational value with a Unit (which may be
    /// Unit::None) and flags.
    Numeric(Numeric),
    /// "(a/b) and a/b differs semantically.  Parens means the value
    /// should be evaluated numerically if possible, without parens /
    /// is not allways division.
    /// The boolean tells if the paren itself should be kept for output.
    Paren(Box<Value>, bool),
    /// A variable reference to be loaded when the value is evaluated.
    Variable(Name, SourcePos),
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
    BinOp(Box<BinOp>),
    /// A unary operator and its operand.
    UnaryOp(Operator, Box<Value>),
    /// A map in sass source is just a list of key/value parirs.
    /// Actual map behaviour comes after evaluating it.
    Map(Vec<(Value, Value)>),
    /// The magic value "&", expanding to the current selectors.
    HereSelector,
    /// A unicode range for font selections. U+NN, U+N?, U+NN-MM.
    /// The string is the entire value, including the "U+" tag.
    UnicodeRange(String),
}

impl Value {
    /// Create a new scalar value.
    pub fn scalar(v: impl Into<Number>) -> Self {
        Self::Numeric(Numeric::scalar(v))
    }

    /// All values other than `False` and `Null` should be considered true.
    pub fn is_true(&self) -> bool {
        !matches!(self, Self::False | Self::Null)
    }

    /// Return true if this value is null.
    ///
    /// Note that an empty unquoted string and a list containing no
    /// non-null values is also considered null.
    pub fn is_null(&self) -> bool {
        match *self {
            Self::Null => true,
            Self::List(ref list, _, false) => list.iter().all(Self::is_null),
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
        Ok(match self {
            Self::Bang(s) => css::Value::Bang(s.clone()),
            Self::Literal(s) => s.evaluate(scope)?.into(),
            Self::Paren(v, expl) => {
                let v = v.do_evaluate(scope, !expl)?;
                if *expl
                    || v == css::Value::Null
                    || matches!(&v, css::Value::Literal(s) if s.is_css_fn())
                    || matches!(&v, css::Value::Call(name, _) if name == "var")
                {
                    css::Value::Paren(Box::new(v))
                } else {
                    v
                }
            }
            Self::Color(rgba, name) => {
                css::Value::Color(rgba.clone().into(), name.clone())
            }
            Self::Variable(name, pos) => scope
                .get(name)
                .map_err(|e| e.at(pos.clone()))?
                .into_calculated(),
            Self::List(v, s, b) => css::Value::List(
                v.iter()
                    .map(|v| v.do_evaluate(scope.clone(), false))
                    .collect::<Result<_, _>>()?,
                *s,
                *b,
            ),
            Self::Call(name, args, pos) => {
                if name.single_raw() == Some("if") {
                    // Magic: `if` is the only function that evaluates its
                    // arguments lazily.  So it is implemented inline here.
                    return if args
                        .evaluate_single(scope.clone(), name!(condition), 0)?
                        .is_true()
                    {
                        args.evaluate_single(scope, name!(if_true), 1)
                    } else {
                        args.evaluate_single(scope, name!(if_false), 2)
                    };
                }
                let call = args.evaluate(scope.clone());

                if let Some(name) = name.single_raw() {
                    let call = call.map_err(|e| e.called_from(pos, name))?;
                    let nname = Name::from(name);
                    if let Some(f) = scope
                        .get_function(&nname)
                        .map_err(|e| e.at(pos.clone()))?
                        .or_else(|| Function::get_builtin(&nname).cloned())
                    {
                        f.call(call).map_err(|e| e.called_from(pos, name))?
                    } else {
                        css::Value::Call(name.to_string(), call.args)
                    }
                } else {
                    let name = name.evaluate(scope)?.value().to_string();
                    let call = call.map_err(|e| e.called_from(pos, &name))?;
                    css::Value::Call(name, call.args)
                }
            }
            Self::Numeric(num) => {
                css::Value::Numeric(num.clone(), arithmetic)
            }
            Self::Map(m) => {
                let mut items = css::ValueMap::new();
                for (k, v) in m.iter() {
                    let k = k.do_evaluate(scope.clone(), arithmetic)?;
                    let v = v.do_evaluate(scope.clone(), arithmetic)?;
                    if items.insert(k, v).is_some() {
                        return Err(Error::S("Duplicate key.".to_string()));
                    }
                }
                css::Value::Map(items)
            }
            Self::Null => css::Value::Null,
            Self::True => css::Value::True,
            Self::False => css::Value::False,
            Self::BinOp(binop) => binop.eval(&scope, arithmetic)?,
            Self::UnaryOp(op, v) => {
                let value = v.do_evaluate(scope, true)?;
                match (op, value) {
                    (Operator::Not, css::Value::Numeric(v, _)) => {
                        (v.value == 0.into()).into()
                    }
                    (Operator::Not, css::Value::True) => css::Value::False,
                    (Operator::Not, css::Value::False) => css::Value::True,
                    (Operator::Minus, css::Value::Numeric(v, _)) => {
                        css::Value::Numeric(-&v, true)
                    }
                    (Operator::Plus, css::Value::Numeric(v, _)) => {
                        css::Value::Numeric(v, true)
                    }
                    (op, css::Value::Literal(s)) if s.quotes().is_none() => {
                        format!("{op}{s}").into()
                    }
                    (op, v) => css::Value::UnaryOp(*op, Box::new(v)),
                }
            }
            Self::HereSelector => {
                scope.get_selectors().get_backref().clone().into()
            }
            Self::UnicodeRange(s) => css::Value::UnicodeRange(s.clone()),
        })
    }

    /// Write a string representation of this value
    ///
    /// This does _not_ evaluate the value.
    pub fn inspect(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use fmt::Display;
        match *self {
            Self::Bang(ref s) => write!(out, "!{s}"),
            Self::Literal(ref s) => {
                if let Some(s) = s.single_raw() {
                    out.write_str(s)
                } else {
                    write!(out, "{s:?}")
                }
            }
            Self::Paren(ref v, _expl) => {
                out.write_str("(")?;
                v.inspect(out)?;
                out.write_str(")")
            }
            Self::Color(ref rgba, ref name) => {
                if let Some(name) = name {
                    out.write_str(name)
                } else {
                    crate::value::Color::from(rgba.clone())
                        .format(Format::introspect())
                        .fmt(out)
                }
            }
            Self::Variable(ref name, ref _pos) => {
                write!(out, "${name}")
            }
            Self::List(ref v, s, b) => {
                if b {
                    out.write_str("(")?;
                }
                if let Some((first, rest)) = v.split_first() {
                    first.inspect(out)?;
                    let s = s.unwrap_or(ListSeparator::Space).sep(false);
                    for i in rest {
                        out.write_str(s)?;
                        i.inspect(out)?;
                    }
                }
                if b {
                    out.write_str(")")?;
                }
                Ok(())
            }
            Self::Call(ref name, ref args, ref _pos) => {
                write!(out, "{name}({args:?})")
            }
            Self::Numeric(ref num) => {
                num.format(Format::introspect()).fmt(out)
            }
            Self::Map(ref m) => {
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
            Self::Null => out.write_str("null"),
            Self::True => out.write_str("true"),
            Self::False => out.write_str("false"),
            Self::BinOp(ref binop) => Inspect(binop).fmt(out),
            Self::UnaryOp(ref op, ref v) => {
                op.fmt(out)?;
                v.inspect(out)
            }
            Self::HereSelector => out.write_str("&"),
            Self::UnicodeRange(ref s) => s.fmt(out),
        }
    }
}

impl From<Numeric> for Value {
    fn from(num: Numeric) -> Self {
        Self::Numeric(num)
    }
}

/// A binary operation.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct BinOp {
    a: Value,
    s1: bool,
    op: Operator,
    s2: bool,
    b: Value,
    pos: SourcePos,
}

impl BinOp {
    pub(crate) fn new(
        a: Value,
        s1: bool,
        op: Operator,
        s2: bool,
        b: Value,
        pos: SourcePos,
    ) -> Self {
        Self {
            a,
            s1,
            op,
            s2,
            b,
            pos,
        }
    }
    fn eval(
        &self,
        scope: &ScopeRef,
        arithmetic: bool,
    ) -> Result<css::Value, Error> {
        if self.op == Operator::And {
            let a = self.a.do_evaluate(scope.clone(), true)?;
            Ok(if a.is_true() {
                self.b.do_evaluate(scope.clone(), true)?
            } else {
                a
            })
        } else if self.op == Operator::Or {
            let a = self.a.do_evaluate(scope.clone(), true)?;
            Ok(if a.is_true() {
                a
            } else {
                self.b.do_evaluate(scope.clone(), true)?
            })
        } else if self.op.is_cmp() {
            let aa = self.a.do_evaluate(scope.clone(), true)?;
            let ba = self.b.do_evaluate(scope.clone(), true)?;
            self.op
                .eval(aa, ba)
                .map_err(|e| match e {
                    BadOp::UndefinedOperation => Invalid::AtError(format!(
                        "Undefined operation \"{}\".",
                        Inspect(self)
                    )),
                    BadOp::Invalid(e) => Invalid::AtError(e.to_string()),
                })
                .at(&self.pos)?
                .ok_or(())
                .or_else(|()| {
                    let a = self.a.do_evaluate(scope.clone(), arithmetic)?;
                    let b = self.b.do_evaluate(scope.clone(), arithmetic)?;
                    Ok(css::BinOp::new(a, self.s1, self.op, self.s2, b)
                        .into())
                })
        } else {
            let (a, b) = {
                let arithmetic = arithmetic || (self.op != Operator::Div);
                let aa = self.a.do_evaluate(scope.clone(), arithmetic)?;
                let b = self.b.do_evaluate(
                    scope.clone(),
                    arithmetic || aa.is_calculated(),
                )?;
                if !arithmetic && b.is_calculated() && !aa.is_calculated() {
                    (self.a.do_evaluate(scope.clone(), true)?, b)
                } else {
                    (aa, b)
                }
            };
            Ok(self
                .op
                .eval(a.clone(), b.clone())
                .map_err(|e| match e {
                    BadOp::UndefinedOperation => Invalid::AtError(format!(
                        "Undefined operation \"{}\".",
                        Inspect(self)
                    )),
                    BadOp::Invalid(e) => Invalid::AtError(e.to_string()),
                })
                .at(&self.pos)?
                .unwrap_or_else(|| {
                    let sx = match self.op {
                        Operator::Div => false,
                        Operator::Minus => {
                            a.type_name() != "string"
                                && b.type_name() != "string"
                        }
                        _ => true,
                    };
                    css::BinOp::new(
                        a,
                        self.s1 && sx,
                        self.op,
                        self.s2 && sx,
                        b,
                    )
                    .into()
                }))
        }
    }
}

impl From<BinOp> for Value {
    fn from(value: BinOp) -> Self {
        Self::BinOp(Box::new(value))
    }
}

struct Inspect<'a>(&'a BinOp);

impl<'a> fmt::Display for Inspect<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.a.inspect(f)?;
        if self.0.s1 {
            f.write_char(' ')?;
        }
        self.0.op.fmt(f)?;
        if self.0.s2 {
            f.write_char(' ')?;
        }
        self.0.b.inspect(f)
    }
}
