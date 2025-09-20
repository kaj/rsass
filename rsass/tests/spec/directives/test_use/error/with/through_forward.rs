//! Tests auto-converted from "sass-spec/spec/directives/use/error/with/through_forward.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("through_forward")
        .mock_file("as/_forwarded.scss", "$a: d !default;\n")
        .mock_file("as/_used.scss", "@forward \"forwarded\" as c-*;\n")
        .mock_file("hide/_forwarded.scss", "$a: d !default;\n")
        .mock_file("hide/_used.scss", "@forward \"forwarded\" hide $a;\n")
        .mock_file("show/_forwarded.scss", "$a: d !default;\n")
        .mock_file("show/_used.scss", "@forward \"forwarded\" show $b;\n")
        .mock_file("with/_forwarded.scss", "$a: d !default;\n")
        .mock_file(
            "with/_used.scss",
            "@forward \"forwarded\" with ($a: c);\n",
        )
}

#[test]
#[ignore] // missing error
fn test_as() {
    let runner = runner().with_cwd("as");
    assert_eq!(
        runner.err(
            "@use \"used\" with ($a: b);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"used\" with ($a: b);\
         \n  |                   ^^^^^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn hide() {
    let runner = runner().with_cwd("hide");
    assert_eq!(
        runner.err(
            "@use \"used\" with ($a: b);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"used\" with ($a: b);\
         \n  |                   ^^^^^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn show() {
    let runner = runner().with_cwd("show");
    assert_eq!(
        runner.err(
            "@use \"used\" with ($a: b);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"used\" with ($a: b);\
         \n  |                   ^^^^^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn with() {
    let runner = runner().with_cwd("with");
    assert_eq!(
        runner.err(
            "@use \"used\" with ($a: b);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"used\" with ($a: b);\
         \n  |                   ^^^^^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
    );
}
