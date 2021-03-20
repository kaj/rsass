//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2123/test-01.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(".class{background-image:url(//foo.bar/favicon.ico)}")
            .unwrap(),
        ".class {\
        \n  background-image: url(//foo.bar/favicon.ico);\
        \n}\
        \n"
    );
}
