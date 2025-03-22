//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2444.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2444")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  @at-root (with: rule) {\
             \n    b: c;\
             \n  }\
             \n}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
