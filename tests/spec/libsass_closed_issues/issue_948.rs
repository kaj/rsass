//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_948.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass("foo { bar: 10 * 5#{px}; }").unwrap(),
        "foo {\
        \n  bar: 50 px;\
        \n}\
        \n"
    );
}
