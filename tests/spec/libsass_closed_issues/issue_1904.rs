//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1904.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  &--#{\'bar\'} {\
            \n    color: red;\
            \n  }\
            \n}"
        )
        .unwrap(),
        ".foo--bar {\
        \n  color: red;\
        \n}\
        \n"
    );
}
