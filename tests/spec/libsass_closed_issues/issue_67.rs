//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_67.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass("foo {bar: 70% - 40%}").unwrap(),
        "foo {\
        \n  bar: 30%;\
        \n}\
        \n"
    );
}
