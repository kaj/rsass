use rsass::input::{FsContext, SourceFile, SourceName};
use rsass::output::Format;
use rsass::sass::{FormalArgs, Function, Name};
use rsass::value::{Number, Numeric, Rgba};
use rsass::*;
use std::sync::Arc;

#[test]
fn simple_value() {
    let parsed = mock_stdin("p { color: $color }").parse().unwrap();
    let format = output::Format {
        style: output::Style::Compressed,
        precision: 5,
    };
    let scope = ScopeRef::new_global(format);
    scope
        .define(Name::from_static("color"), Rgba::from_rgb(0, 0, 0).into())
        .unwrap();
    let mut file_context = FsContext::for_cwd();
    assert_eq!(
        String::from_utf8(
            format.write_root(parsed, scope, &mut file_context).unwrap()
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
    let scope = ScopeRef::new_global(format);
    scope.define_function(
        Name::from_static("get_answer"),
        Function::builtin(
            "",
            &Name::from_static("get_answer"),
            FormalArgs::none(),
            Arc::new(|_| Ok(css::Value::scalar(42))),
        ),
    );
    let parsed = mock_stdin("p { x: get_answer(); }").parse().unwrap();
    let mut file_context = FsContext::for_cwd();
    assert_eq!(
        String::from_utf8(
            format.write_root(parsed, scope, &mut file_context).unwrap()
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
    let scope = ScopeRef::new_global(Default::default());
    scope.define_function(
        Name::from_static("halfway"),
        Function::builtin(
            "",
            &Name::from_static("halfway"),
            FormalArgs::new(vec![
                ("a".into(), None),
                ("b".into(), Some(sass::Value::scalar(0))),
            ]),
            Arc::new(|s| {
                let a = s.get(&"a".into())?.numeric_value().map_err(|v| {
                    Error::BadArgument(
                        "a".into(),
                        format!(
                            "{} is not a number",
                            v.format(Format::introspect())
                        ),
                    )
                })?;
                let b = s.get(&"b".into())?.numeric_value().map_err(|v| {
                    Error::BadArgument(
                        "b".into(),
                        format!(
                            "{} is not a number",
                            v.format(Format::introspect())
                        ),
                    )
                })?;
                if a.unit == b.unit || b.unit.is_none() {
                    Ok(Numeric::new(avg(a.value, b.value), a.unit).into())
                } else if a.unit.is_none() {
                    Ok(Numeric::new(avg(a.value, b.value), b.unit).into())
                } else {
                    Err(Error::error("Incopatible args."))
                }
            }),
        ),
    );
    let parsed = mock_stdin("p { x: halfway(10, 18); }").parse().unwrap();
    let format = output::Format {
        style: output::Style::Compressed,
        precision: 5,
    };
    let mut file_context = FsContext::for_cwd();
    assert_eq!(
        String::from_utf8(
            format.write_root(parsed, scope, &mut file_context).unwrap()
        )
        .unwrap(),
        "p{x:14}\n"
    );
}

fn mock_stdin(data: &str) -> SourceFile {
    SourceFile::scss_bytes(data.as_bytes(), SourceName::root("-"))
}
