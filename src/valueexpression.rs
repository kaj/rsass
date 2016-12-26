use nom::{alphanumeric, multispace};
use std::str::from_utf8;

/// A sass value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Literal(String),
    Variable(String),
    Multi(Vec<Value>),
}

// This should support complex exressions, will get complicated.
named!(pub value_expression<&[u8], Value>,
       chain!(v: many0!(single_value),
              || if v.len() == 1 {
                  v[0].clone()
              } else {
                  Value::Multi(v)
              }));

named!(single_value<&[u8], Value>,
       alt!(chain!(tag!("$") ~ name: alphanumeric ~ multispace?,
                   || Value::Variable(from_utf8(name).unwrap().to_string())) |
            chain!(val: is_not!(";$ \n\t") ~ multispace?,
                   || Value::Literal(from_utf8(val).unwrap().to_string()))));

#[cfg(test)]
mod test {
    use nom::IResult::*;
    use valueexpression::*;

    #[test]
    fn simple_value_literal() {
        assert_eq!(value_expression(b"red;"),
                   Done(&b";"[..], Value::Literal("red".into())))
    }

    #[test]
    fn simple_value_variable() {
        assert_eq!(value_expression(b"$red;"),
                   Done(&b";"[..], Value::Variable("red".into())))
    }
}
