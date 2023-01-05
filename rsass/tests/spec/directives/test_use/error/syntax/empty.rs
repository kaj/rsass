//! Tests auto-converted from "sass-spec/spec/directives/use/error/syntax/empty.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("empty")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@use;\n"),
        "Error: Expected string.\
         \n  ,\
         \n1 | @use;\
         \n  |     ^\
         \n  \'\
         \n  input.scss 1:5  root stylesheet",
    );
}
