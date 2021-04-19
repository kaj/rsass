//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/mixin-in-mixin.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin mix() {\r\
             \n  @mixin foo() {}\r\
             \n}\r\
             \nfoo {\r\
             \n  bar: include mix();\r\
             \n}"
        )
        .unwrap_err(),
        "Error: Mixins may not contain mixin declarations.\
         \n  ,\
         \n2 |   @mixin foo() {}\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
