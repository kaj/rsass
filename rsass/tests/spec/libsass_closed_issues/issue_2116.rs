//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2116.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2116")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo() {\
             \n  @return if(& != null, green, red);\
             \n}\n\
             \ntest {\
             \n  color: foo();\
             \n}\n"),
        "test {\
         \n  color: green;\
         \n}\n"
    );
}
