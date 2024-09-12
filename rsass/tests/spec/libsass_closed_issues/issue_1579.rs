//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1579.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1579")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@function foo($a, $b: null, $c: false) {\
             \n  @return $c;\
             \n}\n\
             \n@function bar($args...) {\
             \n  @return meta.call(foo, $args...);\
             \n}\n\
             \ntest {\
             \n  test: bar(3, $c: true);\
             \n}\n"),
        "test {\
         \n  test: true;\
         \n}\n"
    );
}
