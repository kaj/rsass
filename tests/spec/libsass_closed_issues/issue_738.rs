//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_738.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  &--bar { color: red; }\
            \n  &--1bar { color: blue;}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo--bar {\
        \n  color: red;\
        \n}\
        \n.foo--1bar {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
