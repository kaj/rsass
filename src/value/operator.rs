use crate::css::Value;
use crate::value::{ListSeparator, Numeric, Quotes};
use std::fmt;

/// An operator that can be used in a sass value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    /// The boolean (value propagating) `and` operator.
    And,
    /// The boolean (value propagating) `or` operator.
    Or,
    /// The `==` operator.
    Equal,
    /// The `!=` operator.
    NotEqual,
    /// The `>` operator.
    Greater,
    /// The `>=` operator.
    GreaterE,
    /// The `<` operator.
    Lesser,
    /// The `<=` operator.
    LesserE,

    /// The `+` operator.  Also unary positive.
    Plus,
    /// The `-` operator.  Also unary negative.
    Minus,
    /// The `*` operator.
    Multiply,
    /// The `/` operator.
    Div,
    /// The `%` operator.
    Modulo,

    /// The unary `not` operator.
    Not,
}

impl Operator {
    /// Evaluate this operator with two operands.
    ///
    /// Some operations cannot be evaluated but should remain as is.
    /// In that case, eval returns None.
    pub fn eval(&self, a: Value, b: Value) -> Option<Value> {
        match *self {
            Operator::And => Some(if !a.is_true() { a } else { b }),
            Operator::Or => Some(if a.is_true() { a } else { b }),
            Operator::Equal => Some(Value::from(a == b)),
            Operator::NotEqual => Some(Value::from(a != b)),
            Operator::Greater => Some(Value::from(a > b)),
            Operator::GreaterE => Some(Value::from(a >= b)),
            Operator::Lesser => Some(Value::from(a < b)),
            Operator::LesserE => Some(Value::from(a <= b)),
            Operator::Plus => match (a, b) {
                (Value::Color(a, _), Value::Numeric(b, _))
                    if b.is_no_unit() =>
                {
                    let b = b.as_ratio().ok()?;
                    Some((a.to_rgba().as_ref() + b).into())
                }
                (Value::Color(a, _), Value::Color(b, _)) => {
                    Some((a.to_rgba().as_ref() + b.to_rgba().as_ref()).into())
                }
                (Value::Numeric(a, _), Value::Numeric(b, _)) => {
                    if a.unit == b.unit || b.is_no_unit() {
                        Some(Numeric::new(a.value + b.value, a.unit).into())
                    } else if a.is_no_unit() {
                        Some(Numeric::new(a.value + b.value, b.unit).into())
                    } else if let Some(scaled) = b.as_unitset(&a.unit) {
                        Some(Numeric::new(a.value + scaled, a.unit).into())
                    } else {
                        None
                    }
                }
                (Value::Literal(a, Quotes::None), Value::Literal(b, _)) => {
                    Some(Value::Literal(format!("{}{}", a, b), Quotes::None))
                }
                (Value::Literal(a, _), Value::Literal(b, _)) => Some(
                    Value::Literal(format!("{}{}", a, b), Quotes::Double),
                ),
                (Value::Literal(a, q), b) => Some(Value::Literal(
                    format!("{}{}", a, b.format(Default::default())),
                    q,
                )),
                (a, Value::Literal(b, q)) => Some(Value::Literal(
                    format!("{}{}", a.format(Default::default()), b),
                    q,
                )),
                _ => None,
            },
            Operator::Minus => match (a, b) {
                (Value::Color(a, _), Value::Numeric(b, _))
                    if b.is_no_unit() =>
                {
                    let b = b.as_ratio().ok()?;
                    Some((a.to_rgba().as_ref() - b).into())
                }
                (Value::Color(a, _), Value::Color(b, _)) => {
                    Some((a.to_rgba().as_ref() - b.to_rgba().as_ref()).into())
                }
                (Value::Numeric(a, _), Value::Numeric(b, _)) => {
                    if a.unit == b.unit || b.is_no_unit() {
                        Some(Numeric::new(&a.value - &b.value, a.unit).into())
                    } else if a.is_no_unit() {
                        Some(Numeric::new(&a.value - &b.value, b.unit).into())
                    } else if let Some(scaled) = b.as_unitset(&a.unit) {
                        Some(Numeric::new(&a.value - &scaled, a.unit).into())
                    } else {
                        None
                    }
                }
                // Note: This very special case should probably be much
                // more general.
                (a @ Value::UnicodeRange(..), b @ Value::Literal(..)) => {
                    Some(Value::List(
                        vec![a, Value::UnaryOp(Operator::Minus, Box::new(b))],
                        Some(ListSeparator::Space),
                        false,
                    ))
                }
                _ => None,
            },
            Operator::Multiply => match (a, b) {
                (Value::Numeric(ref a, _), Value::Numeric(ref b, _)) => {
                    Some((a * b).into())
                }
                _ => None,
            },
            Operator::Div if a.is_calculated() || b.is_calculated() => {
                match (a, b) {
                    (Value::Color(a, _), Value::Numeric(b, _))
                        if b.is_no_unit() =>
                    {
                        let bn = b.as_ratio().ok()?;
                        Some((a.to_rgba().as_ref() / bn).into())
                    }
                    (Value::Numeric(ref a, _), Value::Numeric(ref b, _)) => {
                        Some((a / b).into())
                    }
                    (a, b) => Some(Value::BinOp(
                        Box::new(a),
                        false,
                        Operator::Div,
                        false,
                        Box::new(b),
                    )),
                }
            }
            Operator::Div => None,
            Operator::Modulo => match (&a, &b) {
                (&Value::Numeric(ref a, _), &Value::Numeric(ref b, _)) => {
                    if a.unit == b.unit {
                        Some(Value::scalar(&a.value % &b.value))
                    } else if b.is_no_unit() {
                        Some(
                            Numeric::new(&a.value % &b.value, a.unit.clone())
                                .into(),
                        )
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Operator::Not => panic!("not is a unary operator only"),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        out.write_str(match *self {
            Operator::And => "and",
            Operator::Or => "or",
            Operator::Equal => "==",
            Operator::NotEqual => "!=",
            Operator::Greater => ">",
            Operator::GreaterE => ">=",
            Operator::Lesser => "<",
            Operator::LesserE => "<=",
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiply => "*",
            Operator::Modulo => "%",
            Operator::Div => "/",
            Operator::Not => "not",
        })
    }
}
