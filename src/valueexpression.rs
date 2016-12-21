use nom::{alphanumeric};
use nom::IResult::*;
use std::str::from_utf8;

/// A sass value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Literal(String),
    Variable(String),
}

// This should support complex exressions, will get complicated.
// For now, only literal value or single variable reference.
named!(pub value_expression<&[u8], Value>,
       alt!(chain!(tag!("$") ~ name: alphanumeric,
                   || Value::Variable(from_utf8(name).unwrap().into())) |
            chain!(val: is_not!(";"),
                   || Value::Literal(from_utf8(val).unwrap().into()))));

#[test]
fn test_simple_value_literal() {
    assert_eq!(value_expression(b"red;"),
               Done(&b";"[..], Value::Literal("red".into())))
}

#[test]
fn test_simple_value_variable() {
    assert_eq!(value_expression(b"$red;"),
               Done(&b";"[..], Value::Variable("red".into())))
}
