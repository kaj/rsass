use crate::css;
use crate::error::Error;
use crate::functions::get_builtin_function;
use crate::ordermap::OrderMap;
use crate::sass::{CallArgs, SassString};
use crate::value::{ListSeparator, Number, Operator, Quotes, Rgba, Unit};
use crate::variablescope::Scope;
use num_rational::Rational;
use num_traits::Zero;

/// A sass value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    /// A special kind of escape.  Only really used for !important.
    Bang(String),
    /// A call has a name and an argument (which may be multi).
    Call(SassString, CallArgs),
    Literal(SassString),
    /// A comma- or space separated list of values, with or without brackets.
    List(Vec<Value>, ListSeparator, bool, bool),
    /// A Numeric value is a rational value with a Unit (which may be
    /// Unit::None) and flags.
    Numeric(Number, Unit),
    /// "(a/b) and a/b differs semantically.  Parens means the value
    /// should be evaluated numerically if possible, without parens /
    /// is not allways division.
    Paren(Box<Value>),
    Variable(String),
    /// Both a numerical and original string representation,
    /// since case and length should be preserved (#AbC vs #aabbcc).
    Color(Rgba, Option<String>),
    Null,
    True,
    False,
    /// A binary operation, two operands and an operator.
    /// The boolean represents possible whitespace.
    BinOp(Box<Value>, bool, Operator, bool, Box<Value>),
    UnaryOp(Operator, Box<Value>),
    Map(OrderMap<Value, Value>),
    /// The magic value "&", exanding to the current selectors.
    HereSelector,
    /// A unicode range for font selections. U+NN, U+N?, U+NN-MM.
    /// The string is the entire value, including the "U+" tag.
    UnicodeRange(String),
}

impl Value {
    pub fn scalar(v: isize) -> Self {
        Value::Numeric(Number::from(v), Unit::None)
    }
    pub fn bool(v: bool) -> Self {
        if v {
            Value::True
        } else {
            Value::False
        }
    }
    pub fn black() -> Self {
        Value::Color(Rgba::from_rgb(0, 0, 0), Some("black".into()))
    }
    pub fn rgba(r: Rational, g: Rational, b: Rational, a: Rational) -> Self {
        Value::Color(Rgba::new(r, g, b, a), None)
    }

    pub fn type_name(&self) -> &'static str {
        match *self {
            Value::Color(..) => "color",
            Value::Literal(..) => "string",
            Value::Numeric(..) => "number",
            Value::List(..) => "list",
            Value::True | Value::False => "bool",
            Value::Null => "null",
            _ => "unknown",
        }
    }

    /// All values other than `False` and `Null` should be considered true.
    pub fn is_true(&self) -> bool {
        match *self {
            Value::False | Value::Null => false,
            _ => true,
        }
    }

    pub fn is_null(&self) -> bool {
        match *self {
            Value::Null => true,
            Value::List(ref list, _, false, _) => {
                list.iter().all(|v| v.is_null())
            }
            _ => false,
        }
    }

    pub fn evaluate(&self, scope: &dyn Scope) -> Result<css::Value, Error> {
        self.do_evaluate(scope, false)
    }
    pub fn do_evaluate(
        &self,
        scope: &dyn Scope,
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
            Value::Paren(ref v) => v.do_evaluate(scope, true),
            Value::Color(ref rgba, ref name) => {
                Ok(css::Value::Color(rgba.clone(), name.clone()))
            }
            Value::Variable(ref name) => {
                Ok(scope.get(name)?.into_calculated())
            }
            Value::List(ref v, ref s, b, needs_requote) => {
                let items = v
                    .iter()
                    .map(|v| -> Result<css::Value, Error> {
                        let v = v.do_evaluate(scope, false)?;
                        if needs_requote {
                            Ok(v.unrequote())
                        } else {
                            Ok(v)
                        }
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                Ok(css::Value::List(items, s.clone(), b))
            }
            Value::Call(ref name, ref args) => {
                let args = args.evaluate(scope, true)?;
                if let Some(name) = name.single_raw() {
                    match scope.call_function(name, &args) {
                        Some(value) => Ok(value?),
                        None => get_builtin_function(name)
                            .map(|f| f.call(scope, &args))
                            .unwrap_or_else(|| {
                                Ok(css::Value::Call(name.to_string(), args))
                            }),
                    }
                } else {
                    let (name, _) = name.evaluate(scope)?;
                    Ok(css::Value::Call(name, args))
                }
            }
            Value::Numeric(ref num, ref unit) => {
                let mut num = num.clone();
                if arithmetic {
                    num.lead_zero = true;
                }
                Ok(css::Value::Numeric(num, unit.clone(), arithmetic))
            }
            Value::Map(ref m) => {
                let items = m.iter()
                    .map(|&(ref k, ref v)| -> Result<(css::Value, css::Value), Error> {
                        Ok((
                            k.do_evaluate(scope, false)?,
                            v.do_evaluate(scope, false)?,
                        ))
                    })
                    .collect::<Result<OrderMap<_, _>, Error>>()?;
                Ok(css::Value::Map(items))
            }
            Value::Null => Ok(css::Value::Null),
            Value::True => Ok(css::Value::True),
            Value::False => Ok(css::Value::False),
            Value::BinOp(ref a, s1, ref op, s2, ref b) => {
                let (a, b) = {
                    let arithmetic = arithmetic | (*op != Operator::Div);
                    let aa = a.do_evaluate(scope, arithmetic)?;
                    let b = b.do_evaluate(
                        scope,
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
                        s1,
                        op.clone(),
                        s2,
                        Box::new(b),
                    )
                }))
            }
            Value::UnaryOp(ref op, ref v) => {
                let value = v.do_evaluate(scope, true)?;
                match (op.clone(), value) {
                    (Operator::Not, css::Value::Numeric(v, ..)) => {
                        Ok(css::Value::bool(v.is_zero()))
                    }
                    (Operator::Not, css::Value::True) => {
                        Ok(css::Value::False)
                    }
                    (Operator::Not, css::Value::False) => {
                        Ok(css::Value::True)
                    }
                    (Operator::Minus, css::Value::Numeric(v, u, _)) => {
                        Ok(css::Value::Numeric(-&v, u, true))
                    }
                    (Operator::Plus, css::Value::Numeric(v, u, _)) => {
                        Ok(css::Value::Numeric(
                            Number {
                                value: v.value,
                                plus_sign: true,
                                lead_zero: v.lead_zero,
                            },
                            u,
                            true,
                        ))
                    }
                    (op, v) => Ok(css::Value::UnaryOp(op, Box::new(v))),
                }
            }
            Value::HereSelector => Ok(scope.get_selectors().to_value()),
            Value::UnicodeRange(ref s) => {
                Ok(css::Value::UnicodeRange(s.clone()))
            }
        }
    }
}
