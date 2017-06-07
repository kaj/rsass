use rsass::{GlobalScope, value_expression};
use std::str::from_utf8;

mod addition;
mod logic_eq;
mod binary_and_unary;
mod unquote;

pub fn check_eval(expression: &str, expected: &str) {
    let mut scope = GlobalScope::new();
    let expression = format!("{};", expression);
    let (end, foo) = value_expression(expression.as_bytes()).unwrap();
    println!("Expression is: {:?}", foo);
    assert_eq!(Ok(";"), from_utf8(end));
    let result = foo.evaluate(&mut scope);
    println!(" ... evals to: {:?}", result);
    assert_eq!(format!("{}", result), expected)
}
