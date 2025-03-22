//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1171.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1171")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@function foo($initial, $args...) {\
             \n  $args: list.append($args, 3);\n\
             \n  @return bar($initial, $args...);\
             \n}\n\
             \n@function bar($args...) {\
             \n  @return list.length($args);\
             \n}\n\
             \n@function baz($initial, $args...) {\
             \n  $args: list.append($args, 3);\n\
             \n  @return list.nth($args, 1);\
             \n}\n\
             \n.test {\
             \n  foo: foo(1, 2);\
             \n  baz: baz(1, 2);\
             \n}"),
        ".test {\
         \n  foo: 3;\
         \n  baz: 2;\
         \n}\n"
    );
}
