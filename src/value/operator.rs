use css::Value;
use num_rational::Rational;
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

    Not,
}

impl Operator {
    pub fn eval(&self, a: Value, b: Value) -> Value {
        match *self {
            Operator::And => Value::bool(a.is_true() && b.is_true()),
            Operator::Or => if a.is_true() {
                a
            } else {
                b
            },
            Operator::Equal => Value::bool(equal_values(&a, &b)),
            Operator::NotEqual => Value::bool(!equal_values(&a, &b)),
            Operator::Greater => Value::bool(a > b),
            Operator::GreaterE => Value::bool(a >= b),
            Operator::Lesser => Value::bool(a < b),
            Operator::LesserE => Value::bool(a <= b),
            Operator::Plus => {
                match (a, b) {
                    (
                        Value::Color(r, g, b, a, _),
                        Value::Numeric(n, Unit::None, ..),
                    ) => Value::rgba(r + n, g + n, b + n, a),
                    (
                        Value::Color(ar, ag, ab, aa, _),
                        Value::Color(br, bg, bb, ba, _),
                    ) => {
                        // TODO Sum or average the alpha?
                        Value::rgba(ar + br, ag + bg, ab + bb, aa + ba)
                    }
                    (
                        Value::Numeric(a, au, ..),
                        Value::Numeric(b, bu, ..),
                    ) => {
                        if au == bu || bu == Unit::None {
                            Value::Numeric(a + b, au, false, true)
                        } else if au == Unit::None {
                            Value::Numeric(a + b, bu, false, true)
                        } else {
                            Value::Literal(
                                format!("{}{}", a, b),
                                Quotes::None,
                            )
                        }
                    }
                    (
                        Value::Literal(a, Quotes::None),
                        Value::Literal(b, _),
                    ) => Value::Literal(format!("{}{}", a, b), Quotes::None),
                    (Value::Literal(a, _), Value::Literal(b, _)) => {
                        Value::Literal(format!("{}{}", a, b), Quotes::Double)
                    }
                    (Value::Literal(a, q), b) => {
                        Value::Literal(format!("{}{}", a, b), q)
                    }
                    (a, Value::Literal(b, q)) => {
                        Value::Literal(format!("{}{}", a, b), q)
                    }
                    (a, b) => {
                        Value::BinOp(Box::new(a), Operator::Plus, Box::new(b))
                    }
                }
            }
            Operator::Minus => match (&a, &b) {
                (
                    &Value::Color(ref r, ref g, ref b, ref a, _),
                    &Value::Numeric(ref n, Unit::None, ..),
                ) => Value::rgba(r - n, g - n, b - n, *a),
                (
                    &Value::Color(ref ar, ref ag, ref ab, ref aa, _),
                    &Value::Color(ref br, ref bg, ref bb, ref ba, _),
                ) => Value::rgba(ar - br, ag - bg, ab - bb, avg(aa, ba)),
                (
                    &Value::Numeric(ref av, ref au, ..),
                    &Value::Numeric(ref bv, ref bu, ..),
                ) => {
                    if au == bu || bu == &Unit::None {
                        Value::Numeric(av - bv, au.clone(), false, true)
                    } else if au == &Unit::None {
                        Value::Numeric(av - bv, bu.clone(), false, true)
                    } else {
                        Value::BinOp(
                            Box::new(a.clone()),
                            Operator::Minus,
                            Box::new(b.clone()),
                        )
                    }
                }
                _ => Value::BinOp(
                    Box::new(a.clone()),
                    Operator::Minus,
                    Box::new(b.clone()),
                ),
            },
            Operator::Multiply => {
                if let (
                    &Value::Numeric(ref a, ref au, ..),
                    &Value::Numeric(ref b, ref bu, ..),
                ) = (&a, &b)
                {
                    if bu == &Unit::None {
                        Value::Numeric(a * b, au.clone(), false, true)
                    } else if au == &Unit::None {
                        Value::Numeric(a * b, bu.clone(), false, true)
                    } else {
                        Value::Literal(format!("{}*{}", a, b), Quotes::None)
                    }
                } else {
                    Value::Literal(format!("{}*{}", a, b), Quotes::None)
                }
            }
            Operator::Not => panic!("not is a unary operator only"),
        }
        // Fallback, might be needed later:
        // Value::BinOp(Box::new(a), self.clone(), Box::new(b))
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
