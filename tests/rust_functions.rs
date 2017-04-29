extern crate rsass;
use rsass::{FileContext, GlobalScope, OutputStyle, SassFunction, Scope, Value,
            parse_scss_data};
use std::sync::Arc;

#[test]
fn simple_value() {
    let mut scope = GlobalScope::new();
    scope.define("color", &Value::black());
    let parsed = parse_scss_data(b"p { color: $color }").unwrap();
    let style = OutputStyle::Compressed;
    let file_context = FileContext::new();
    assert_eq!(style
                   .write_root(&parsed, &mut scope, file_context)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .unwrap(),
               "p{color:black}\n");
}

#[test]
fn simple_function() {
    let mut scope = GlobalScope::new();
    scope.define_function(
        "get_answer",
        SassFunction::builtin(
            vec![],
            false,
            Arc::new(|_| Ok(Value::scalar(42)))));
    let parsed = parse_scss_data(b"p { x: get_answer(); }").unwrap();
    let style = OutputStyle::Compressed;
    let file_context = FileContext::new();
    assert_eq!(style
                   .write_root(&parsed, &mut scope, file_context)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .unwrap(),
               "p{x:42}\n");
}
