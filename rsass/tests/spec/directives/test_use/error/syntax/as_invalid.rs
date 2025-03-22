//! Tests auto-converted from "sass-spec/spec/directives/use/error/syntax/as_invalid.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("as_invalid")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@use \"foo\" as 1;\n"),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | @use \"foo\" as 1;\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
    );
}
