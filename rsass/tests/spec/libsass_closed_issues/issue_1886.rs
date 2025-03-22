//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1886.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1886")
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
