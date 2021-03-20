//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_641.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(".#{\"foo\"}--1 { width:100%; }").unwrap(),
        ".foo--1 {\
        \n  width: 100%;\
        \n}\
        \n"
    );
}
