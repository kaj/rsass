//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1568.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1568")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("body {\
             \n    font-weight: bold; // test\
             \n    font-size: 10px // test\
             \n}\n"),
        "body {\
         \n  font-weight: bold;\
         \n  font-size: 10px;\
         \n}\n"
    );
}
