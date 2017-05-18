//! Tests from
//! `sass-spec/spec/parser/operations/binary_and_unary`.

use super::super::super::value_expression;
use std::str::from_utf8;
use variablescope::GlobalScope;

#[test]
fn minus_before_minus() {
    check("- 1 - 2", "-3")
}
#[test]
fn minus_after_minus() {
    check("1 - - 2", "3")
}
#[test]
fn plus_before_minus() {
    check("+ 1 - 2", "-1")
}
#[test]
fn plus_after_minus() {
    check("1 - + 2", "-1")
}
#[test]
fn not_before_plus() {
    check("not 1 + 2", "false2")
}
#[test]
fn not_after_plus() {
    check("1 + not 2", "1false")
}

fn check(expression: &str, expected: &str) {
    let mut scope = GlobalScope::new();
    let expression = format!("{};", expression);
    let (end, foo) = value_expression(expression.as_bytes()).unwrap();
    println!("Expression is: {:?}", foo);
    assert_eq!(Ok(";"), from_utf8(end));
    let result = foo.evaluate(&mut scope);
    println!(" ... evals to: {:?}", result);
    assert_eq!(format!("{}", result),
               expected)
}
