//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2123/test-01.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("test-01")
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
