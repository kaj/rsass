//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1303.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1303")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \n.simple {\
             \n  a: selector.replace(\'foo.bar\', \'foo\', \'foo[baz]\');\
             \n}\n"),
        ".simple {\
         \n  a: foo.bar[baz];\
         \n}\n"
    );
}
