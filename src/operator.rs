use std::fmt;
use valueexpression::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    Equal,
    NotEqual,
}

impl Operator {
    pub fn eval(&self, a: Value, b: Value) -> Value {
        println!("Eval {} {} {}", a, self, b);
        match self {
            &Operator::Equal => if a == b { Value::True } else { Value::False },
            &Operator::NotEqual => {
                if a != b { Value::True } else { Value::False }
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
                   &Operator::Equal => "==",
                   &Operator::NotEqual => "!=",
               })
    }
}
