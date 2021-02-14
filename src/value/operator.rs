use crate::css::Value;
use crate::value::{ListSeparator, Quotes, Unit};
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
                (Value::Color(a, _), Value::Numeric(bn, Unit::None, _)) => {
                    let bn = bn.as_ratio().ok()?;
                    Some((a.to_rgba().as_ref() + bn).into())
                }
                (Value::Color(a, _), Value::Color(b, _)) => {
                    Some((a.to_rgba().as_ref() + b.to_rgba().as_ref()).into())
                }
                (Value::Numeric(a, au, ..), Value::Numeric(b, bu, ..)) => {
                    if au == bu || bu == Unit::None {
                        Some(Value::Numeric(a + b, au, true))
                    } else if au == Unit::None {
                        Some(Value::Numeric(a + b, bu, true))
                    } else if let Some(scale) = bu.scale_to(&au) {
                        Some(Value::Numeric(a + b * scale, au, true))
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
                (Value::Color(a, _), Value::Numeric(bn, Unit::None, _)) => {
                    let bn = bn.as_ratio().ok()?;
                    Some((a.to_rgba().as_ref() - bn).into())
                }
                (Value::Color(a, _), Value::Color(b, _)) => {
                    Some((a.to_rgba().as_ref() - b.to_rgba().as_ref()).into())
                }
                (Value::Numeric(av, au, ..), Value::Numeric(bv, bu, ..)) => {
                    if au == bu || bu == Unit::None {
                        Some(Value::Numeric(&av - &bv, au, true))
                    } else if au == Unit::None {
                        Some(Value::Numeric(&av - &bv, bu, true))
                    } else if let Some(scale) = bu.scale_to(&au) {
                        Some(Value::Numeric(
                            &av - &(bv * scale),
                            au.clone(),
                            true,
                        ))
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
                    &Value::Numeric(ref a, ref au, ..),
                    &Value::Numeric(ref b, ref bu, ..),
                ) = (&a, &b)
                {
                    if bu == &Unit::None {
                        Some(Value::Numeric(a * b, au.clone(), true))
                    } else if au == &Unit::None {
                        Some(Value::Numeric(a * b, bu.clone(), true))
                    } else if bu == &Unit::Percent {
                        Some(Value::Numeric(a * b / 100, au.clone(), true))
                    } else if au == &Unit::Percent {
                        Some(Value::Numeric(a * b / 100, bu.clone(), true))
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
                        (
                            Value::Color(a, _),
                            Value::Numeric(bn, Unit::None, ..),
                        ) => {
                            let bn = bn.as_ratio().ok()?;
                            Some((a.to_rgba().as_ref() / bn).into())
                        }
                        (
                            Value::Numeric(av, au, ..),
                            Value::Numeric(bv, bu, ..),
                        ) => {
                            if bu == Unit::None {
                                Some(Value::Numeric(&av / &bv, au, true))
                            } else if au == bu {
                                Some(Value::Numeric(
                                    &av / &bv,
                                    Unit::None,
                                    true,
                                ))
                            } else if let Some(scale) = bu.scale_to(&au) {
                                Some(Value::Numeric(
                                    &av / &(bv * scale),
                                    Unit::None,
                                    true,
                                ))
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
                (
                    &Value::Numeric(ref av, ref au, ..),
                    &Value::Numeric(ref bv, ref bu, ..),
                ) => {
                    if au == bu {
                        Some(Value::Numeric(av % bv, Unit::None, true))
                    } else if bu == &Unit::None {
                        Some(Value::Numeric(av % bv, au.clone(), true))
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
