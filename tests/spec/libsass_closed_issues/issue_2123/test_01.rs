//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2123/test-01.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".class{background-image:url(//foo.bar/favicon.ico)}"),
        ".class {\
         \n  background-image: url(//foo.bar/favicon.ico);\
         \n}\n"
    );
}
