//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1570.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1570")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n    font: 12px/normal serif;\
             \n}\n\
             \nb {\
             \n    font: normal 12px/normal serif;\
             \n}\n"),
        "a {\
         \n  font: 12px/normal serif;\
         \n}\
         \nb {\
         \n  font: normal 12px/normal serif;\
         \n}\n"
    );
}
