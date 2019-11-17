//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/function-in-mixin.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin mix() {\r\
             \n  @function foo() {}\r\
             \n}\r\
             \nfoo {\r\
             \n  bar: include mix();\r\
             \n}"
        )
        .unwrap_err(),
        "Error: Mixins may not contain function declarations.\
         \n  ,\
         \n2 |   @function foo() {}\
         \n  |   ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet\
         \n",
    );
}
