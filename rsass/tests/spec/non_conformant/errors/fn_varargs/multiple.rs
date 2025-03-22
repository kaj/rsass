//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-varargs/multiple.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("multiple")
}

#[test]
fn test() {
    assert_eq!(
        runner().err("@function test($param...,$rest...) {}"),
        "Error: expected \")\".\
         \n  ,\
         \n1 | @function test($param...,$rest...) {}\
         \n  |                          ^\
         \n  \'\
         \n  input.scss 1:26  root stylesheet",
    );
}
