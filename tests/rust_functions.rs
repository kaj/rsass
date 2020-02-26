use rsass::*;
use std::sync::Arc;

#[test]
fn simple_value() {
    let parsed = parse_scss_data(b"p { color: $color }").unwrap();
    let format = OutputFormat {
        style: OutputStyle::Compressed,
        precision: 5,
    };
    let mut scope = GlobalScope::new(format);
    scope.define("color", &css::Value::black());
    let file_context = FileContext::new();
    assert_eq!(
        format
            .write_root(&parsed, &mut scope, &file_context)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        "p{color:black}\n"
    );
}

#[test]
fn simple_function() {
    let format = OutputFormat {
        style: OutputStyle::Compressed,
        precision: 5,
    };
    let mut scope = GlobalScope::new(format);
    scope.define_function(
        "get_answer",
        SassFunction::builtin(
            vec![],
            false,
            Arc::new(|_| Ok(css::Value::scalar(42))),
        ),
    );
    let parsed = parse_scss_data(b"p { x: get_answer(); }").unwrap();
    let file_context = FileContext::new();
    assert_eq!(
        format
            .write_root(&parsed, &mut scope, &file_context)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        "p{x:42}\n"
    );
}

#[cfg(test)]
fn avg(a: Number, b: Number) -> Number {
    Number::from((a.value + b.value) / 2)
}

#[test]
fn function_with_args() {
    let mut scope = GlobalScope::new(Default::default());
    scope.define_function(
        "halfway",
        SassFunction::builtin(
            vec![
                ("a".into(), sass::Value::Null),
                ("b".into(), sass::Value::scalar(0)),
            ],
            false,
            Arc::new(|s| match (s.get("a")?, s.get("b")?) {
                (
                    css::Value::Numeric(a, au, ..),
                    css::Value::Numeric(b, bu, ..),
                ) => {
                    if au == bu || bu == Unit::None {
                        Ok(css::Value::Numeric(avg(a, b), au, true))
                    } else if au == Unit::None {
                        Ok(css::Value::Numeric(avg(a, b), bu, true))
                    } else {
                        Err(Error::BadArguments("Incopatible args".into()))
                    }
                }
                (a, b) => {
                    Err(Error::badargs(&["number", "number"], &[&a, &b]))
                }
            }),
        ),
    );
    let parsed = parse_scss_data(b"p { x: halfway(10, 18); }").unwrap();
    let format = OutputFormat {
        style: OutputStyle::Compressed,
        precision: 5,
    };
    let file_context = FileContext::new();
    assert_eq!(
        format
            .write_root(&parsed, &mut scope, &file_context)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        "p{x:14}\n"
    );
}
