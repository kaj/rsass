//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-varargs/at-start.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("at-start")
}

#[test]
fn test() {
    assert_eq!(
        runner().err("@function test($rest...,$param) {}"),
        "Error: expected \")\".\
         \n  ,\
         \n1 | @function test($rest...,$param) {}\
         \n  |                         ^\
         \n  \'\
         \n  input.scss 1:25  root stylesheet",
    );
}
