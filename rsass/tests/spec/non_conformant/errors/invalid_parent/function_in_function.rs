//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/function-in-function.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function-in-function")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@function foo () {\r\
             \n  @function bar() {}\r\
             \n}"
        ),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @function bar() {}\
         \n  |   ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
