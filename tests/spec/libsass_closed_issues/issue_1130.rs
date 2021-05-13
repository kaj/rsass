//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1130.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($args...) {\
             \n  @return bar($args...);\
             \n}\n\
             \n@function bar() {\
             \n @return \"hi\";\
             \n}\n\
             \n.foo {\
             \n  result: foo();\
             \n}\n"),
        ".foo {\
         \n  result: \"hi\";\
         \n}\n"
    );
}
