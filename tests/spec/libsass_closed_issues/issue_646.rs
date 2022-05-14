//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_646.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_646")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo() {\
             \n  /* $bar: 1; */\
             \n @return true;\
             \n}\n\
             \nfoo {\
             \n  foo: foo();\
             \n}\n"),
        "foo {\
         \n  foo: true;\
         \n}\n"
    );
}
