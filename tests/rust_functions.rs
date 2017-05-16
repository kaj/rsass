extern crate rsass;
use rsass::*;
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
    scope.define_function("get_answer",
                          SassFunction::builtin(vec![],
                                                false,
                                                Arc::new(|_| {
        Ok(Value::scalar(42))
    })));
    let parsed = parse_scss_data(b"p { x: get_answer(); }").unwrap();
    let style = OutputStyle::Compressed;
    let file_context = FileContext::new();
    assert_eq!(style
                   .write_root(&parsed, &mut scope, file_context)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .unwrap(),
               "p{x:42}\n");
}

#[test]
fn function_with_args() {
    let mut scope = GlobalScope::new();
    scope.define_function("halfway",
                          SassFunction::builtin(vec![("a".into(),
                                                      Value::Null),
                                                     ("b".into(),
                                                      Value::scalar(0))],
                                                false,
                                                Arc::new(|s| {
        let half = Rational::new(1, 2);
        match (s.get("a"), s.get("b")) {
            (Value::Numeric(a, au, ..), Value::Numeric(b, bu, ..)) => {
                if au == bu || bu == Unit::None {
                    Ok(Value::Numeric((a + b) * half, au, false, true))
                } else if au == Unit::None {
                    Ok(Value::Numeric((a + b) * half, bu, false, true))
                } else {
                    Err(Error::BadArguments("Incopatible args".into()))
                }
            }
            (a, b) => Err(Error::badargs(&["number", "number"], &[&a, &b])),
        }
    })));
    let parsed = parse_scss_data(b"p { x: halfway(10, 18); }").unwrap();
    let style = OutputStyle::Compressed;
    let file_context = FileContext::new();
    assert_eq!(style
                   .write_root(&parsed, &mut scope, file_context)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .unwrap(),
               "p{x:14}\n");
}
