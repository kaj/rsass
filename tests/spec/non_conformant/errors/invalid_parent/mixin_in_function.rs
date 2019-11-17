//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/mixin-in-function.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo () {\r\
             \n  @mixin bar() {}\r\
             \n}"
        )
        .unwrap_err(),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @mixin bar() {}\
         \n  |   ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet\
         \n",
    );
}
