use num_rational::Rational;
use std::fmt;
use unit::Unit;
use valueexpression::{Quotes, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
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
}

impl Operator {
    pub fn eval(&self, a: Value, b: Value) -> Value {
        match *self {
            Operator::And => Value::bool(a.is_true() && b.is_true()),
            Operator::Or => Value::bool(a.is_true() || b.is_true()),
            Operator::Equal => Value::bool(a == b),
            Operator::NotEqual => Value::bool(a != b),
            Operator::Greater => Value::bool(a > b),
            Operator::GreaterE => Value::bool(a >= b),
            Operator::Lesser => Value::bool(a < b),
            Operator::LesserE => Value::bool(a <= b),
            Operator::Plus => {
                match (a, b) {
                    (Value::Color(r, g, b, a, _),
                     Value::Numeric(n, Unit::None, _)) => {
                        Value::rgba(r + n, g + n, b + n, a)
                    }
                    (Value::Color(ar, ag, ab, aa, _),
                     Value::Color(br, bg, bb, ba, _)) => {
                        // TODO Sum or average the alpha?
                        Value::rgba(ar + br, ag + bg, ab + bb, aa + ba)
                    }
                    (Value::Numeric(a, au, _), Value::Numeric(b, bu, _)) => {
                        if au == bu || bu == Unit::None {
                            Value::Numeric(a + b, au, true)
                        } else if au == Unit::None {
                            Value::Numeric(a + b, bu, true)
                        } else {
                            Value::Literal(format!("{}{}", a, b), Quotes::None)
                        }
                    }
                    (Value::Literal(a, q), Value::Literal(b, _)) => {
                        Value::Literal(format!("{}{}", a, b), q)
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
            Operator::Minus => {
                match (&a, &b) {
                    (&Value::Color(ref r, ref g, ref b, ref a, _),
                     &Value::Numeric(ref n, ref u, _)) if u == &Unit::None => {
                        Value::rgba(r - n, g - n, b - n, *a)
                    }
                    (&Value::Color(ref ar, ref ag, ref ab, ref aa, _),
                     &Value::Color(ref br, ref bg, ref bb, ref ba, _)) => {
                        Value::rgba(ar - br, ag - bg, ab - bb, avg(aa, ba))
                    }
                    (&Value::Numeric(ref av, ref au, _),
                     &Value::Numeric(ref bv, ref bu, _)) => {
                        if au == bu || bu == &Unit::None {
                            Value::Numeric(av - bv, au.clone(), true)
                        } else if au == &Unit::None {
                            Value::Numeric(av - bv, bu.clone(), true)
                        } else {
                            Value::BinOp(Box::new(a.clone()),
                                         Operator::Minus,
                                         Box::new(b.clone()))
                        }
                    }
                    _ => {
                        Value::BinOp(Box::new(a.clone()),
                                     Operator::Minus,
                                     Box::new(b.clone()))
                    }
                }
            }
            Operator::Multiply => {
                if let (&Value::Numeric(ref a, ref au, _),
                        &Value::Numeric(ref b, ref bu, _)) = (&a, &b) {
                    if bu == &Unit::None {
                        Value::Numeric(a * b, au.clone(), true)
                    } else if au == &Unit::None {
                        Value::Numeric(a * b, bu.clone(), true)
                    } else {
                        Value::Literal(format!("{}*{}", a, b), Quotes::None)
                    }
                } else {
                    Value::Literal(format!("{}*{}", a, b), Quotes::None)
                }
            }
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
        write!(out,
               "{}",
               match self {
                   &Operator::And => "and",
                   &Operator::Or => "or",
                   &Operator::Equal => "==",
                   &Operator::NotEqual => "!=",
                   &Operator::Greater => ">",
                   &Operator::GreaterE => ">=",
                   &Operator::Lesser => "<",
                   &Operator::LesserE => "<=",
                   &Operator::Plus => "+",
                   &Operator::Minus => "-",
                   &Operator::Multiply => "*",
               })
    }
}
