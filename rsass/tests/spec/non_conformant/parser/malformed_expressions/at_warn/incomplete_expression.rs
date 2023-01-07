//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/malformed_expressions/at-warn/incomplete-expression.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("incomplete-expression")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@warn(\"foo\";\n"),
        "Error: expected \")\".\
         \n  ,\
         \n1 | @warn(\"foo\";\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
