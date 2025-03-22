//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1645.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1645")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@function foo($a, $should-be-empty...) {\
             \n  @return list.length($should-be-empty);\
             \n}\n\
             \n@function bar($args...) {\
             \n  @return meta.call(foo, $args...);\
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
