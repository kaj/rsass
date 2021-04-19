//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1093/argument/mixin.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($bar:#{}) {\
             \n  @return $bar;\
             \n}\
             \n\
             \nfoo {\
             \n  @include foo;\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Expected expression.\
         \n  ,\
         \n1 | @mixin foo($bar:#{}) {\
         \n  |                 ^^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
    );
}
