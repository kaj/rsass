//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/mixin-in-mixin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@mixin mix() {\r\
             \n  @mixin foo() {}\r\
             \n}\r\
             \nfoo {\r\
             \n  bar: include mix();\r\
             \n}"
        ),
        "Error: Mixins may not contain mixin declarations.\
         \n  ,\
         \n2 |   @mixin foo() {}\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
