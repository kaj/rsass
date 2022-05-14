//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1295.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1295")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  $nothing: null;\
             \n  foo: \"#{$nothing}\' %\' \'#{$nothing}\";\
             \n}\n"),
        "foo {\
         \n  foo: \"\' %\' \'\";\
         \n}\n"
    );
}
