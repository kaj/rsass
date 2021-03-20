//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1171.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($initial, $args...) {\
            \n  $args: append($args, 3);\
            \n\
            \n  @return bar($initial, $args...);\
            \n}\
            \n\
            \n@function bar($args...) {\
            \n  @return length($args);\
            \n}\
            \n\
            \n@function baz($initial, $args...) {\
            \n  $args: append($args, 3);\
            \n\
            \n  @return nth($args, 1);\
            \n}\
            \n\
            \n.test {\
            \n  foo: foo(1, 2);\
            \n  baz: baz(1, 2);\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  foo: 3;\
        \n  baz: 2;\
        \n}\
        \n"
    );
}
