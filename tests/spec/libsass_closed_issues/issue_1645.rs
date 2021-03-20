//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1645.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($a, $should-be-empty...) {\
            \n  @return length($should-be-empty);\
            \n}\
            \n\
            \n@function bar($args...) {\
            \n  @return call(foo, $args...);\
            \n}\
            \n\
            \n@function args($args...) {\
            \n  @return $args;\
            \n}\
            \n\
            \n$a: args(1, 2, 3);\
            \n\
            \ntest {\
            \n  test: bar($a);\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  test: 0;\
        \n}\
        \n"
    );
}
