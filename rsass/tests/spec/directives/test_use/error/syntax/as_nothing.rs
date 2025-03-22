//! Tests auto-converted from "sass-spec/spec/directives/use/error/syntax/as_nothing.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("as_nothing")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@use \"foo\" as;\n"),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | @use \"foo\" as;\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
    );
}
