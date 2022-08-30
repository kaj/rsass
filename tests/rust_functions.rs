use rsass::input::{FsContext, SourceFile, SourceName};
use rsass::output::{Format, Style};
use rsass::sass::{CallError, FormalArgs, Function, Name};
use rsass::value::{Numeric, Rgba};
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
                let a: Numeric = s.get("a".into())?;
                let b: Numeric = s.get("b".into())?;
                let unit = if a.unit == b.unit || b.unit.is_none() {
                    a.unit
                } else if a.unit.is_none() {
                    b.unit
                } else {
                    return Err(CallError::msg("Incopatible args."));
                };
                Ok(Numeric::new((a.value + b.value) / 2, unit).into())
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
