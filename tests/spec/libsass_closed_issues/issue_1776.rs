//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1776.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "h1 {\r\
            \n  width :calc(100% - 110px);\r\
            \n}"
        )
        .unwrap(),
        "h1 {\
        \n  width: calc(100% - 110px);\
        \n}\
        \n"
    );
}
