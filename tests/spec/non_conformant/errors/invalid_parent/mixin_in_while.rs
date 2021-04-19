//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/mixin-in-while.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@while (false) {\r\
             \n  @mixin foo() {}\r\
             \n}"
        )
        .unwrap_err(),
        "Error: Mixins may not be declared in control directives.\
         \n  ,\
         \n2 |   @mixin foo() {}\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
