//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1061.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  &.div,\
            \n  &.span {\
            \n    display: block;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a.div, a.span {\
        \n  display: block;\
        \n}\
        \n"
    );
}
