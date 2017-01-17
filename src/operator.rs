use num_rational::Rational;
use std::fmt;
use std::ops::Neg;
use valueexpression::{Unit, Value};

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
                        Value::Color(add(r, n),
                                     add(g, n),
                                     add(b, n),
                                     a.clone(),
                                     None)
                    }
                    (&Value::Color(ref ar, ref ag, ref ab, ref aa, _),
                     &Value::Color(ref br, ref bg, ref bb, ref ba, _)) => {
                        Value::Color(add8(ar, br),
                                     add8(ag, bg),
                                     add8(ab, bb),
                                     aa + ba, // TODO or average?
                                     None)
                    }
                    (&Value::Numeric(ref a, ref au, _),
                     &Value::Numeric(ref b, ref bu, _)) => {
                        if au == bu || bu == &Unit::None {
                            Value::Numeric(a + b, au.clone(), true)
                        } else if au == &Unit::None {
                            Value::Numeric(a + b, bu.clone(), true)
                        } else {
                            Value::Literal(format!("{}{}", a, b), false)
                        }
                    }
                    (a, b) => Value::Literal(format!("{}{}", a, b), false),
                }
            }
            &Operator::Minus => {
                match (&a, &b) {
                    (&Value::Color(ref r, ref g, ref b, ref a, _),
                     &Value::Numeric(ref n, ref u, _)) if u == &Unit::None => {
                        let n = n.neg();
                        Value::Color(add(r, &n),
                                     add(g, &n),
                                     add(b, &n),
                                     a.clone(),
                                     None)
                    }
                    (&Value::Color(ref ar, ref ag, ref ab, ref aa, _),
                     &Value::Color(ref br, ref bg, ref bb, ref ba, _)) => {
                        Value::Color(sub8(ar, br),
                                     sub8(ag, bg),
                                     sub8(ab, bb),
                                     (aa + ba) / Rational::from_integer(2),
                                     None)
                    }
                    (&Value::Numeric(ref av, ref au, _),
                     &Value::Numeric(ref bv, ref bu, _)) => {
                        if au == bu || bu == &Unit::None {
                            Value::Numeric(av - bv, au.clone(), true)
                        } else if au == &Unit::None {
                            Value::Numeric(av - bv, bu.clone(), true)
                        } else {
                            Value::Literal(format!("{}-{}", a, b), false)
                        }
                    }
                    (a, b) => Value::Literal(format!("{}-{}", a, b), false),
                }
            }
        }
        // Fallback, might be needed later:
        // Value::BinOp(Box::new(a), self.clone(), Box::new(b))
    }
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

fn add(x: &u8, y: &Rational) -> u8 {
    let v = *x as f32 + *y.numer() as f32 / *y.denom() as f32;
    if v < 0.0 {
        0
    } else if v > 255.0 {
        0xff
    } else {
        v as u8
    }
}
fn add8(x: &u8, y: &u8) -> u8 {
    match x.overflowing_add(*y) {
        (_, true) => 0xff,
        (s, false) => s,
    }
}

fn sub8(x: &u8, y: &u8) -> u8 {
    if *x > *y { *x - *y } else { 0 }
}
