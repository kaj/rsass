//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1093/parameter/mixin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@mixin foo($bar) {\
             \n  a: $bar;\
             \n}\n\
             \nfoo {\
             \n  @include foo(#{});\
             \n}\n"
        ),
        "Error: Expected expression.\
         \n  ,\
         \n6 |   @include foo(#{});\
         \n  |                ^^\
         \n  \'\
         \n  input.scss 6:16  root stylesheet",
    );
}
