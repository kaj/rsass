//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_201.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass("a, b, { color: red; }").unwrap(),
        "a, b {\
        \n  color: red;\
        \n}\
        \n"
    );
}
