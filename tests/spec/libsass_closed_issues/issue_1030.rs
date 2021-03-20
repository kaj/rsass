//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1030.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin will-change() {\
            \n  @supports (will-change: transform) {\
            \n    will-change: transform;\
            \n  }\
            \n}\
            \ndiv {\
            \n  a {\
            \n    top: 10px;\
            \n    @include will-change();\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div a {\
        \n  top: 10px;\
        \n}\
        \n@supports (will-change: transform) {\
        \n  div a {\
        \n    will-change: transform;\
        \n  }\
        \n}\
        \n"
    );
}
