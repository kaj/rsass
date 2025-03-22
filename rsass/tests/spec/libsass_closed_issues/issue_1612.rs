//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1612.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1612")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("c {\
             \n  b: \"foo\", bar;\
             \n  b: \"foo\", bar\
             \n}\n"),
        "c {\
         \n  b: \"foo\", bar;\
         \n  b: \"foo\", bar;\
         \n}\n"
    );
}
