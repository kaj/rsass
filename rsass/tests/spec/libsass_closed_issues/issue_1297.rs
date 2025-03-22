//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1297.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1297")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".test .testa {\
             \n  @at-root #{\"%foo\"} {\
             \n    color: red;\
             \n  }\
             \n  @extend %foo;\
             \n}\n"),
        ".test .testa {\
         \n  color: red;\
         \n}\n"
    );
}
