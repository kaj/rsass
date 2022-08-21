use rsass::input::{FsContext, SourceFile, SourceName};
use rsass::output::{Format, Style};
use rsass::sass::{CallError, FormalArgs, Function, Name};
use rsass::value::{Number, Numeric, Rgba};
use rsass::*;
use std::sync::Arc;

#[test]
fn simple_value() -> Result<(), Error> {
    let mut context = FsContext::for_cwd().with_format(Format {
        style: Style::Compressed,
        precision: 5,
    });
    context
        .get_scope()
        .define(Name::from_static("color"), Rgba::from_rgb(0, 0, 0).into())?;
    let output = context.transform(mock_stdin("p { color: $color }"))?;
    assert_eq!(String::from_utf8(output).unwrap(), "p{color:#000}\n");
    Ok(())
}

#[test]
fn simple_function() -> Result<(), Error> {
    let mut context = FsContext::for_cwd().with_format(Format {
        style: Style::Compressed,
        precision: 5,
    });
    context.get_scope().define_function(
        Name::from_static("get_answer"),
        Function::builtin(
            "",
            &Name::from_static("get_answer"),
            FormalArgs::none(),
            Arc::new(|_| Ok(css::Value::scalar(42))),
        ),
    );
    let output = context.transform(mock_stdin("p { x: get_answer(); }"))?;
    assert_eq!(String::from_utf8(output).unwrap(), "p{x:42}\n");
    Ok(())
}

#[cfg(test)]
fn avg(a: Number, b: Number) -> Number {
    (a + b) / 2
}

#[test]
fn function_with_args() -> Result<(), Error> {
    let mut context = FsContext::for_cwd().with_format(Format {
        style: Style::Compressed,
        precision: 5,
    });
    context.get_scope().define_function(
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
                    CallError::BadArgument(
                        "a".into(),
                        format!(
                            "{} is not a number",
                            v.format(Format::introspect())
                        ),
                    )
                })?;
                let b = s.get(&"b".into())?.numeric_value().map_err(|v| {
                    CallError::BadArgument(
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
                    Err(CallError::msg("Incopatible args."))
                }
            }),
        ),
    );
    let output =
        context.transform(mock_stdin("p { x: halfway(10, 18); }"))?;
    assert_eq!(String::from_utf8(output).unwrap(), "p{x:14}\n");
    Ok(())
}

fn mock_stdin(data: &str) -> SourceFile {
    SourceFile::scss_bytes(data.as_bytes(), SourceName::root("-"))
}
