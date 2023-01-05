//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1629.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1629")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  background: url(...) 2rem 3rem / auto 2rem;\
             \n}\n"),
        "foo {\
         \n  background: url(...) 2rem 3rem/auto 2rem;\
         \n}\n"
    );
}
