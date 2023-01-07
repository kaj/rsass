//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2808.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2808")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("test {\
             \n  content: str-slice(abcdef, -10, 2)\
             \n}\n"),
        "test {\
         \n  content: ab;\
         \n}\n"
    );
}
