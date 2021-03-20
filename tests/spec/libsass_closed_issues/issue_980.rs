//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_980.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($value, $default: 13, $args...) {\
            \n  $res: $value + $default;\
            \n  @if length($args) != 0 {\
            \n    $res: $res + nth($args, 1);\
            \n  }\
            \n  @return $res;\
            \n}\
            \n\
            \n.test {\
            \n  value: foo(3); // expected: 16\
            \n  value: foo(3, 4); // expected: 7\
            \n  value: foo(3, 4, 5, 6); // expected: 12\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  value: 16;\
        \n  value: 7;\
        \n  value: 12;\
        \n}\
        \n"
    );
}
