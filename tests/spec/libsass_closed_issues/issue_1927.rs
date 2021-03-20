//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1927.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@media screen {\
            \n  $variable: dynamic;\
            \n  .foo-#{$variable} {a: b}\
            \n  .bar {\
            \n    @extend .foo-dynamic\
            \n  }\
            \n}"
        )
        .unwrap(),
        "@media screen {\
        \n  .foo-dynamic, .bar {\
        \n    a: b;\
        \n  }\
        \n}\
        \n"
    );
}
