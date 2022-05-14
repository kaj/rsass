//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_67.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_67")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {bar: 70% - 40%}"),
        "foo {\
         \n  bar: 30%;\
         \n}\n"
    );
}
