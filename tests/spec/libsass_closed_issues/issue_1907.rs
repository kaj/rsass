//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1907.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1907")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar: \'test\' + \'1 #{2} 3\';\
             \n}"),
        "foo {\
         \n  bar: \"test1 2 3\";\
         \n}\n"
    );
}
