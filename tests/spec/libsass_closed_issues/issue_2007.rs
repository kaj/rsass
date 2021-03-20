//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2007.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo() {\
            \n  @media (mix-width: 100px) {\
            \n    @extend %bar;\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  @media (mix-width: 100px) {\
            \n    %bar {\
            \n      foo: bar;\
            \n    }\
            \n  }\
            \n\
            \n  @include foo;\
            \n}\
            \n"
        )
        .unwrap(),
        "@media (mix-width: 100px) {\
        \n  foo foo {\
        \n    foo: bar;\
        \n  }\
        \n}\
        \n"
    );
}
