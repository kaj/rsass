//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2007.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2007")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@mixin foo() {\
             \n  @media (mix-width: 100px) {\
             \n    @extend %bar;\
             \n  }\
             \n}\n\
             \nfoo {\
             \n  @media (mix-width: 100px) {\
             \n    %bar {\
             \n      foo: bar;\
             \n    }\
             \n  }\n\
             \n  @include foo;\
             \n}\n"),
        "@media (mix-width: 100px) {\
         \n  foo foo {\
         \n    foo: bar;\
         \n  }\
         \n}\n"
    );
}
