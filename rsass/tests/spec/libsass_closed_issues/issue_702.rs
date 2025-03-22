//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_702.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_702")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n.foo {\
             \n  content: meta.function-exists(\"feature-exists\");\
             \n  content: meta.feature-exists(\"foo\");\
             \n}\n"),
        ".foo {\
         \n  content: true;\
         \n  content: false;\
         \n}\n"
    );
}
