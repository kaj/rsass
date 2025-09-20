//! Tests auto-converted from "sass-spec/spec/directives/use/error/with/nested.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("nested")
        .mock_file("_other.scss", "c {$a: d !default}\n")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"other\" with ($a: b);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n1 | @use \"other\" with ($a: b);\
         \n  |                    ^^^^^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
}
