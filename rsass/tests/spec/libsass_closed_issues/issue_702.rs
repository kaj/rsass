//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_702.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_702")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  content: function-exists(\"feature-exists\");\
             \n  content: feature-exists(\"foo\");\
             \n}\n"),
        ".foo {\
         \n  content: true;\
         \n  content: false;\
         \n}\n"
    );
}
