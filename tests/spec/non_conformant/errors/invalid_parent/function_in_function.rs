//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/function-in-function.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo () {\r\
             \n  @function bar() {}\r\
             \n}"
        )
        .unwrap_err(),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @function bar() {}\
         \n  |   ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
