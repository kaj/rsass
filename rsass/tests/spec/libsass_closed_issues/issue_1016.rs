//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1016.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1016")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  [baz=\"#{&}\"] {\
             \n    foo: bar;\
             \n  }\
             \n}\n"),
        ".foo [baz=\".foo\"] {\
         \n  foo: bar;\
         \n}\n"
    );
}
