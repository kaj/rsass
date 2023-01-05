//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/malformed_expressions/at-warn/no-argument.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no-argument")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@warn;\n"),
        "Error: Expected expression.\
         \n  ,\
         \n1 | @warn;\
         \n  |      ^\
         \n  \'\
         \n  input.scss 1:6  root stylesheet",
    );
}
