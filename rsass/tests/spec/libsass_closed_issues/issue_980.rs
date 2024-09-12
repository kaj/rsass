//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_980.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_980")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@function foo($value, $default: 13, $args...) {\
             \n  $res: $value + $default;\
             \n  @if list.length($args) != 0 {\
             \n    $res: $res + list.nth($args, 1);\
             \n  }\
             \n  @return $res;\
             \n}\n\
             \n.test {\
             \n  value: foo(3); // expected: 16\
             \n  value: foo(3, 4); // expected: 7\
             \n  value: foo(3, 4, 5, 6); // expected: 12\
             \n}\n"),
        ".test {\
         \n  value: 16;\
         \n  value: 7;\
         \n  value: 12;\
         \n}\n"
    );
}
