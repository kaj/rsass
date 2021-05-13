//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1886.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("body {\
             \n  background: url()\
             \n}"),
        "body {\
         \n  background: url();\
         \n}\n"
    );
}
