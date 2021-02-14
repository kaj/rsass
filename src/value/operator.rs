use crate::css::Value;
use crate::value::{ListSeparator, Numeric, Quotes, Unit};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    And,
    Or,
    Equal,
    NotEqual,
    Greater,
    GreaterE,
    Lesser,
    LesserE,

    Plus,
    Minus,
    Multiply,
    Div,
    Modulo,

    Not,
}

impl Operator {
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
                    if b.unit == Unit::None =>
                {
                    let b = b.as_ratio().ok()?;
                    Some((a.to_rgba().as_ref() + b).into())
                }
                (Value::Color(a, _), Value::Color(b, _)) => {
                    Some((a.to_rgba().as_ref() + b.to_rgba().as_ref()).into())
                }
                (Value::Numeric(a, _), Value::Numeric(b, _)) => {
                    if a.unit == b.unit || b.unit == Unit::None {
                        Some(Numeric::new(a.value + b.value, a.unit).into())
                    } else if a.unit == Unit::None {
                        Some(Numeric::new(a.value + b.value, b.unit).into())
                    } else if let Some(scaled) = b.as_unit(a.unit.clone()) {
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
                    if b.unit == Unit::None =>
                {
                    let b = b.as_ratio().ok()?;
                    Some((a.to_rgba().as_ref() - b).into())
                }
                (Value::Color(a, _), Value::Color(b, _)) => {
                    Some((a.to_rgba().as_ref() - b.to_rgba().as_ref()).into())
                }
                (Value::Numeric(a, _), Value::Numeric(b, _)) => {
                    if a.unit == b.unit || b.unit == Unit::None {
                        Some(Numeric::new(&a.value - &b.value, a.unit).into())
                    } else if a.unit == Unit::None {
                        Some(Numeric::new(&a.value - &b.value, b.unit).into())
                    } else if let Some(scaled) = b.as_unit(a.unit.clone()) {
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
                        ListSeparator::Space,
                        false,
                    ))
                }
                _ => None,
            },
            Operator::Multiply => {
                if let (
                    &Value::Numeric(ref a, _),
                    &Value::Numeric(ref b, _),
                ) = (&a, &b)
                {
                    if b.unit == Unit::None {
                        Some(
                            Numeric::new(&a.value * &b.value, a.unit.clone())
                                .into(),
                        )
                    } else if a.unit == Unit::None {
                        Some(
                            Numeric::new(&a.value * &b.value, b.unit.clone())
                                .into(),
                        )
                    } else if b.unit == Unit::Percent {
                        Some(
                            Numeric::new(
                                &a.value * &b.value / 100,
                                a.unit.clone(),
                            )
                            .into(),
                        )
                    } else if a.unit == Unit::Percent {
                        Some(
                            Numeric::new(
                                &a.value * &b.value / 100,
                                b.unit.clone(),
                            )
                            .into(),
                        )
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Operator::Div => {
                if a.is_calculated() || b.is_calculated() {
                    match (a, b) {
                        (Value::Color(a, _), Value::Numeric(b, _))
                            if b.unit == Unit::None =>
                        {
                            let bn = b.as_ratio().ok()?;
                            Some((a.to_rgba().as_ref() / bn).into())
                        }
                        (Value::Numeric(a, _), Value::Numeric(b, _)) => {
                            if b.unit == Unit::None {
                                Some(
                                    Numeric::new(&a.value / &b.value, a.unit)
                                        .into(),
                                )
                            } else if a.unit == b.unit {
                                Some(Value::scalar(&a.value / &b.value))
                            } else if let Some(scaled) = b.as_unit(a.unit) {
                                Some(Value::scalar(&a.value / &scaled))
                            } else {
                                None
                            }
                        }
                        //_ => None,
                        (a, b) => Some(Value::BinOp(
                            Box::new(a),
                            false,
                            Operator::Div,
                            false,
                            Box::new(b),
                        )),
                    }
                } else {
                    None
                }
            }
            Operator::Modulo => match (&a, &b) {
                (&Value::Numeric(ref a, _), &Value::Numeric(ref b, _)) => {
                    if a.unit == b.unit {
                        Some(Value::scalar(&a.value % &b.value))
                    } else if b.unit == Unit::None {
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
        write!(
            out,
            "{}",
            match *self {
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
            }
        )
    }
}
