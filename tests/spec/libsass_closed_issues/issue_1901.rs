//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1901.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1901")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a, b {\
             \n    &:not(c) {\
             \n        d: e;\
             \n    }\
             \n}\n"),
        "a:not(c), b:not(c) {\
         \n  d: e;\
         \n}\n"
    );
}
