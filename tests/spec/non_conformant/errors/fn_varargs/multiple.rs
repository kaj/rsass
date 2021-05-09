//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-varargs/multiple.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@function test($param...,$rest...) {}"),
        "Error: expected \")\".\
         \n  ,\
         \n1 | @function test($param...,$rest...) {}\
         \n  |                         ^\
         \n  \'\
         \n  input.scss 1:25  root stylesheet",
    );
}
