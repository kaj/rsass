use css::Value;
use num_rational::Rational;
use num_traits::Zero;
use std::fmt;
use value::{Quotes, Unit};

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

    Not,
}

impl Operator {
    pub fn eval(&self, a: Value, b: Value) -> Option<Value> {
        match *self {
            Operator::And => Some(Value::bool(a.is_true() && b.is_true())),
            Operator::Or => if a.is_true() {
                Some(a)
            } else {
                Some(b)
            },
            Operator::Equal => Some(Value::bool(equal_values(&a, &b))),
            Operator::NotEqual => Some(Value::bool(!equal_values(&a, &b))),
            Operator::Greater => Some(Value::bool(a > b)),
            Operator::GreaterE => Some(Value::bool(a >= b)),
            Operator::Lesser => Some(Value::bool(a < b)),
            Operator::LesserE => Some(Value::bool(a <= b)),
            Operator::Plus => {
                match (a, b) {
                    (
                        Value::Color(r, g, b, a, _),
                        Value::Numeric(n, Unit::None, ..),
                    ) => Some(Value::rgba(r + n, g + n, b + n, a)),
                    (
                        Value::Color(ar, ag, ab, aa, _),
                        Value::Color(br, bg, bb, ba, _),
                    ) => {
                        // TODO Sum or average the alpha?
                        Some(Value::rgba(ar + br, ag + bg, ab + bb, aa + ba))
                    }
                    (
                        Value::Numeric(a, au, ..),
                        Value::Numeric(b, bu, ..),
                    ) => {
                        if au == bu || bu == Unit::None {
                            Some(Value::Numeric(a + b, au, false, true))
                        } else if au == Unit::None {
                            Some(Value::Numeric(a + b, bu, false, true))
                        } else {
                            Some(Value::Literal(
                                format!("{}{}", a, b),
                                Quotes::None,
                            ))
                        }
                    }
                    (
                        Value::Literal(a, Quotes::None),
                        Value::Literal(b, _),
                    ) => Some(Value::Literal(
                        format!("{}{}", a, b),
                        Quotes::None,
                    )),
                    (Value::Literal(a, _), Value::Literal(b, _)) => Some(
                        Value::Literal(format!("{}{}", a, b), Quotes::Double),
                    ),
                    (Value::Literal(a, q), b) => {
                        Some(Value::Literal(format!("{}{}", a, b), q))
                    }
                    (a, Value::Literal(b, q)) => {
                        Some(Value::Literal(format!("{}{}", a, b), q))
                    }
                    _ => None,
                }
            }
            Operator::Minus => match (&a, &b) {
                (
                    &Value::Color(ref r, ref g, ref b, ref a, _),
                    &Value::Numeric(ref n, Unit::None, ..),
                ) => Some(Value::rgba(r - n, g - n, b - n, *a)),
                (
                    &Value::Color(ref ar, ref ag, ref ab, ref aa, _),
                    &Value::Color(ref br, ref bg, ref bb, ref ba, _),
                ) => Some(Value::rgba(
                    ar - br,
                    ag - bg,
                    ab - bb,
                    avg(aa, ba),
                )),
                (
                    &Value::Numeric(ref av, ref au, ..),
                    &Value::Numeric(ref bv, ref bu, ..),
                ) => {
                    if au == bu || bu == &Unit::None {
                        Some(Value::Numeric(av - bv, au.clone(), false, true))
                    } else if au == &Unit::None {
                        Some(Value::Numeric(av - bv, bu.clone(), false, true))
                    } else {
                        None
                    }
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
                        Some(Value::Numeric(a * b, au.clone(), false, true))
                    } else if au == &Unit::None {
                        Some(Value::Numeric(a * b, bu.clone(), false, true))
                    } else {
                        // TODO None?
                        Some(Value::Literal(
                            format!("{}*{}", a, b),
                            Quotes::None,
                        ))
                    }
                } else {
                    // TODO None?
                    Some(Value::Literal(
                        format!("{}*{}", a, b),
                        Quotes::None,
                    ))
                }
            }
            Operator::Div => {
                if a.is_calculated() || b.is_calculated() {
                    match (&a, &b) {
                        (
                            &Value::Color(ref r, ref g, ref b, ref a, _),
                            &Value::Numeric(ref n, Unit::None, ..),
                        ) => Some(Value::rgba(r / n, g / n, b / n, *a)),
                        (
                            &Value::Numeric(ref av, ref au, ..),
                            &Value::Numeric(ref bv, ref bu, ..),
                        ) => {
                            if bv.is_zero() {
                                None
                            } else if bu == &Unit::None {
                                Some(Value::Numeric(
                                    av / bv,
                                    au.clone(),
                                    false,
                                    true,
                                ))
                            } else if au == bu {
                                Some(Value::Numeric(
                                    av / bv,
                                    Unit::None,
                                    false,
                                    true,
                                ))
                            } else {
                                None
                            }
                        }
                        //_ => None,
                        (a, b) => Some(Value::BinOp(
                            Box::new(a.clone()),
                            false, // *space1,
                            Operator::Div,
                            false, // *space2,
                            Box::new(b.clone()),
                        )),
                    }
                } else {
                    None
                }
            }
            Operator::Not => panic!("not is a unary operator only"),
        }
    }
}

fn avg(a: &Rational, b: &Rational) -> Rational {
    (a + b) * Rational::new(1, 2)
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
                Operator::Div => "/",
                Operator::Not => "not",
            }
        )
    }
}

/// A more relaxed equality checker for the sass == operator.
///
/// Strings with equal content are considered equal, even if they have
/// different quoting.
///
/// Make this a separate function, so the rust == operator is still
/// the strict derived version, for unit tests etc.
fn equal_values(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (&Value::Literal(ref a, _), &Value::Literal(ref b, _)) => a == b,
        (a, b) => a == b,
    }
}
