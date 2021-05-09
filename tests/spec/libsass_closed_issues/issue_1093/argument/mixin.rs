//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1093/argument/mixin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@mixin foo($bar:#{}) {\
             \n  @return $bar;\
             \n}\n\
             \nfoo {\
             \n  @include foo;\
             \n}\n"
        ),
        "Error: Expected expression.\
         \n  ,\
         \n1 | @mixin foo($bar:#{}) {\
         \n  |                 ^^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
    );
}
