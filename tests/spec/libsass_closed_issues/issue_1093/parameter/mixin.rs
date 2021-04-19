//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1093/parameter/mixin.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($bar) {\
             \n  a: $bar;\
             \n}\
             \n\
             \nfoo {\
             \n  @include foo(#{});\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Expected expression.\
         \n  ,\
         \n6 |   @include foo(#{});\
         \n  |                ^^\
         \n  \'\
         \n  input.scss 6:16  root stylesheet",
    );
}
