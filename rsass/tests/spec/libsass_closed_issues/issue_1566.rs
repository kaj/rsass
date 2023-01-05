//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1566.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1566")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($predicate) {\
             \n  @return call(\'bar\', $predicate);\
             \n}\n\
             \n@function bar($predicate) {\
             \n  @return type-of($predicate);\
             \n}\n\
             \ntest {\
             \n  test: foo(1 2 3);\
             \n}\n"),
        "test {\
         \n  test: list;\
         \n}\n"
    );
}
