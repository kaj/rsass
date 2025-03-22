//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-varargs/with-default.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("with-default")
}

#[test]
fn test() {
    assert_eq!(
        runner().err("@function test($param...:\"default\") {}"),
        "Error: expected \")\".\
         \n  ,\
         \n1 | @function test($param...:\"default\") {}\
         \n  |                         ^\
         \n  \'\
         \n  input.scss 1:25  root stylesheet",
    );
}
