//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2462.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("b {\
             \n    color: lighten(Crimson, 10%);\
             \n}\n"),
        "b {\
         \n  color: #ed365b;\
         \n}\n"
    );
}
