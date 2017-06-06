use super::*;
use formalargs::CallArgs;
use nom::IResult::*;
use num_rational::Rational;
use num_traits::{One, Zero};
use std::str::from_utf8;
use unit::Unit;
use variablescope::GlobalScope;

mod parser_operations;
mod unquote;

#[test]
fn simple_number() {
    check_expr("4;", number(4, 1))
}

#[test]
fn simple_number_neg() {
    check_expr("-4;", number(-4, 1))
}

#[test]
fn simple_number_pos() {
    check_expr("+4;",
               Value::Numeric(Rational::new(4, 1), Unit::None, true, false))
}

#[test]
fn simple_number_dec() {
    check_expr("4.34;", number(434, 100))
}
#[test]
fn simple_number_onlydec() {
    check_expr(".34;", number(34, 100))
}
#[test]
fn simple_number_onlydec_neg() {
    check_expr("-.34;", number(-34, 100))
}
#[test]
fn simple_number_onlydec_pos() {
    check_expr("+.34;",
               Value::Numeric(Rational::new(34, 100), Unit::None, true, false))
}
#[test]
fn number_and_interpolation_makes_space_list() {
    check_expr("12#{3};",
               Value::List(vec![number(12, 1),
                                Value::Interpolation(
                                    Box::new(number(3, 1)))],
                           ListSeparator::Space))
}

fn number(nom: isize, denom: isize) -> Value {
    Value::Numeric(Rational::new(nom, denom), Unit::None, false, false)
}

#[test]
fn simple_value_literal() {
    check_expr("rad;", Value::Literal("rad".into(), Quotes::None))
}

#[test]
fn strings_misc_quotes() {
    check_expr("foo \"bar\" 'baz';",
               Value::List(vec![Value::Literal("foo".into(), Quotes::None),
                                Value::Literal("bar".into(), Quotes::Double),
                                Value::Literal("baz".into(), Quotes::Single)],
                           ListSeparator::Space))
}

#[test]
fn strings_escaped_quotes() {
    check_expr("\"b'a\\\"r\" 'b\\'a\"z';",
               Value::List(vec![Value::Literal("b'a\"r".into(), Quotes::Double),
                                Value::Literal("b'a\"z".into(), Quotes::Single)
                                ],
                           ListSeparator::Space))
}

#[test]
fn simple_value_literal_color() {
    check_expr("red;",
               Value::Color(Rational::new(255, 1),
                            Rational::zero(),
                            Rational::zero(),
                            Rational::one(),
                            Some("red".into())))
}

#[test]
fn simple_value_variable() {
    check_expr("$red;", Value::Variable("red".into()))
}

#[test]
fn paren_literal() {
    check_expr("(rad);",
               Value::Paren(Box::new(Value::Literal("rad".into(),
                                                    Quotes::None))))
}

#[test]
fn paren_multi() {
    check_expr("(rod bloe);",
               Value::Paren(Box::new(Value::List(
                   vec![Value::Literal("rod".into(), Quotes::None),
                        Value::Literal("bloe".into(), Quotes::None)],
                   ListSeparator::Space))))
}

#[test]
fn paren_multi_comma() {
    check_expr("(rod, bloe);",
               Value::Paren(Box::new(Value::List(
                   vec![Value::Literal("rod".into(), Quotes::None),
                        Value::Literal("bloe".into(), Quotes::None)],
                   ListSeparator::Comma))))
}

#[test]
fn multi_comma() {
    check_expr("rod, bloe;",
               Value::List(vec![Value::Literal("rod".into(), Quotes::None),
                                Value::Literal("bloe".into(), Quotes::None)],
                           ListSeparator::Comma))
}

#[test]
fn paren_multi_comma_trailing() {
    check_expr("(rod, bloe, );",
               Value::Paren(Box::new(Value::List(
                   vec![Value::Literal("rod".into(), Quotes::None),
                        Value::Literal("bloe".into(), Quotes::None)],
                   ListSeparator::Comma))))
}

#[test]
fn multi_comma_trailing() {
    check_expr("rod, bloe, ;",
               Value::List(vec![Value::Literal("rod".into(), Quotes::None),
                                Value::Literal("bloe".into(), Quotes::None)],
                           ListSeparator::Comma))
}

#[test]
fn call_no_args() {
    check_expr("foo();", Value::Call("foo".to_string(), CallArgs::default()))
}

#[test]
fn call_one_arg() {
    check_expr("foo(17);",
               Value::Call("foo".to_string(),
                           CallArgs::new(vec![(None, Value::scalar(17))])))
}

#[test]
fn multi_expression() {
    check_expr("15/10 2 3;",
               Value::List(vec![Value::Div(Box::new(Value::scalar(15)),
                                           Box::new(Value::scalar(10)),
                                           false,
                                           false),
                                Value::scalar(2),
                                Value::scalar(3)],
                           ListSeparator::Space))
}

#[test]
fn double_div() {
    check_expr("15/5/3;",
               Value::Div(Box::new(Value::Div(Box::new(Value::scalar(15)),
                                              Box::new(Value::scalar(5)),
                                              false,
                                              false)),
                          Box::new(Value::scalar(3)),
                          false,
                          false))
}

#[test]
fn color_short() {
    check_expr("#AbC;",
               Value::Color(Rational::new(170, 1),
                            Rational::new(187, 1),
                            Rational::new(204, 1),
                            Rational::one(),
                            Some("#AbC".into())))
}

#[test]
fn color_long() {
    check_expr("#AaBbCc;",
               Value::Color(Rational::new(170, 1),
                            Rational::new(187, 1),
                            Rational::new(204, 1),
                            Rational::one(),
                            Some("#AaBbCc".into())))
}

fn check_expr(expr: &str, value: Value) {
    assert_eq!(value_expression(expr.as_bytes()), Done(&b";"[..], value))
}

#[test]
fn parse_extended_literal() {
    use variablescope::GlobalScope;
    let t = value_expression(b"http://#{\")\"}.com/;");
    if let &Done(ref rest, ref result) = &t {
        assert_eq!(rest, b";");
        println!("Got {:?}", result);
        assert_eq!("http://).com/",
                   format!("{}", result.evaluate(&GlobalScope::new())));
    } else {
        assert_eq!(format!("{:?}", t), "Done")
    }
}

fn check_eval(expression: &str, expected: &str) {
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
