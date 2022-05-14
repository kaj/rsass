//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_456.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_456")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("body {\
             \n  -webkit-filter: invert(100%);\
             \n}\n"),
        "body {\
         \n  -webkit-filter: invert(100%);\
         \n}\n"
    );
}
