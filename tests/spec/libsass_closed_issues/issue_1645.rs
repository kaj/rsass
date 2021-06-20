//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1645.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($a, $should-be-empty...) {\
             \n  @return length($should-be-empty);\
             \n}\n\
             \n@function bar($args...) {\
             \n  @return call(foo, $args...);\
             \n}\n\
             \n@function args($args...) {\
             \n  @return $args;\
             \n}\n\
             \n$a: args(1, 2, 3);\n\
             \ntest {\
             \n  test: bar($a);\
             \n}\n"),
        "test {\
         \n  test: 0;\
         \n}\n"
    );
}
