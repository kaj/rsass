//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/malformed_expressions/at-error/incomplete-expression.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("incomplete-expression")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@error(\"foo\";\n"),
        "Error: expected \")\".\
         \n  ,\
         \n1 | @error(\"foo\";\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
