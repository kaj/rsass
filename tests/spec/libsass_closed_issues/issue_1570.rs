//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1570.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
