//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/mixin-in-function.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin-in-function")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@function foo () {\r\
             \n  @mixin bar() {}\r\
             \n}"
        ),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @mixin bar() {}\
         \n  |   ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
