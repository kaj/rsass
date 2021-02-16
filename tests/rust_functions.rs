use rsass::sass::Name;
use rsass::value::{Number, Numeric, Rgba, Unit};
use rsass::*;
use std::sync::Arc;

#[test]
fn simple_value() {
    let parsed = parse_scss_data(b"p { color: $color }").unwrap();
    let format = output::Format {
        style: output::Style::Compressed,
        precision: 5,
    };
    let mut scope = GlobalScope::new(format);
    scope.define(Name::from_static("color"), &Rgba::from_rgb(0, 0, 0).into());
    let file_context = FsFileContext::new();
    assert_eq!(
        String::from_utf8(
            format
                .write_root(&parsed, &mut scope, &file_context)
                .unwrap()
        )
        .unwrap(),
        "p{color:#000}\n"
    );
}

#[test]
fn simple_function() {
    let format = output::Format {
        style: output::Style::Compressed,
        precision: 5,
    };
    let mut scope = GlobalScope::new(format);
    scope.define_function(
        Name::from_static("get_answer"),
        SassFunction::builtin(
            vec![],
            false,
            Arc::new(|_| Ok(css::Value::scalar(42))),
        ),
    );
    let parsed = parse_scss_data(b"p { x: get_answer(); }").unwrap();
    let file_context = FsFileContext::new();
    assert_eq!(
        String::from_utf8(
            format
                .write_root(&parsed, &mut scope, &file_context)
                .unwrap()
        )
        .unwrap(),
        "p{x:42}\n"
    );
}

#[cfg(test)]
fn avg(a: Number, b: Number) -> Number {
    (a + b) / 2
}

#[test]
fn function_with_args() {
    let mut scope = GlobalScope::new(Default::default());
    scope.define_function(
        Name::from_static("halfway"),
        SassFunction::builtin(
            vec![
                ("a".into(), sass::Value::Null),
                ("b".into(), sass::Value::scalar(0)),
            ],
            false,
            Arc::new(|s| {
                let a = s
                    .get("a")?
                    .numeric_value()
                    .map_err(|v| Error::badarg("number", &v))?;
                let b = s
                    .get("b")?
                    .numeric_value()
                    .map_err(|v| Error::badarg("number", &v))?;
                if a.unit == b.unit || b.unit == Unit::None {
                    Ok(Numeric::new(avg(a.value, b.value), a.unit).into())
                } else if a.unit == Unit::None {
                    Ok(Numeric::new(avg(a.value, b.value), b.unit).into())
                } else {
                    Err(Error::BadArguments("Incopatible args".into()))
                }
            }),
        ),
    );
    let parsed = parse_scss_data(b"p { x: halfway(10, 18); }").unwrap();
    let format = output::Format {
        style: output::Style::Compressed,
        precision: 5,
    };
    let file_context = FsFileContext::new();
    assert_eq!(
        String::from_utf8(
            format
                .write_root(&parsed, &mut scope, &file_context)
                .unwrap()
        )
        .unwrap(),
        "p{x:14}\n"
    );
}
