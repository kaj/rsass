use num_rational::Rational;
use std::fmt;
use valueexpression::{Quotes, Unit, Value};

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
}

impl Operator {
    pub fn eval(&self, a: Value, b: Value) -> Value {
        match self {
            &Operator::And => bool_val(a.is_true() && b.is_true()),
            &Operator::Or => bool_val(a.is_true() || b.is_true()),
            &Operator::Equal => bool_val(a == b),
            &Operator::NotEqual => bool_val(a != b),
            &Operator::Greater => bool_val(a > b),
            &Operator::GreaterE => bool_val(a >= b),
            &Operator::Lesser => bool_val(a < b),
            &Operator::LesserE => bool_val(a <= b),
            &Operator::Plus => {
                match (&a, &b) {
                    (&Value::Color(ref r, ref g, ref b, ref a, _),
                     &Value::Numeric(ref n, ref u, _)) if u == &Unit::None => {
                        Value::rgba(r + n, g + n, b + n, a.clone())
                    }
                    (&Value::Color(ref ar, ref ag, ref ab, ref aa, _),
                     &Value::Color(ref br, ref bg, ref bb, ref ba, _)) => {
                        // TODO Sum or average the alpha?
                        Value::rgba(ar + br, ag + bg, ab + bb, aa + ba)
                    }
                    (&Value::Numeric(ref a, ref au, _),
                     &Value::Numeric(ref b, ref bu, _)) => {
                        if au == bu || bu == &Unit::None {
                            Value::Numeric(a + b, au.clone(), true)
                        } else if au == &Unit::None {
                            Value::Numeric(a + b, bu.clone(), true)
                        } else {
                            Value::Literal(format!("{}{}", a, b), Quotes::None)
                        }
                    }
                    (a, b) => {
                        Value::BinOp(Box::new(a.clone()),
                                     Operator::Plus,
                                     Box::new(b.clone()))
                    }
                }
            }
            &Operator::Minus => {
                match (&a, &b) {
                    (&Value::Color(ref r, ref g, ref b, ref a, _),
                     &Value::Numeric(ref n, ref u, _)) if u == &Unit::None => {
                        Value::rgba(r - n, g - n, b - n, a.clone())
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
                            Value::Literal(format!("{}-{}", a, b), Quotes::None)
                        }
                    }
                    (a, b) => {
                        Value::Literal(format!("{}-{}", a, b), Quotes::None)
                    }
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
               })
    }
}

fn bool_val(v: bool) -> Value {
    if v { Value::True } else { Value::False }
}
