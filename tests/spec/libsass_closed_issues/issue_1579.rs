//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1579.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($a, $b: null, $c: false) {\
            \n  @return $c;\
            \n}\
            \n\
            \n@function bar($args...) {\
            \n  @return call(foo, $args...);\
            \n}\
            \n\
            \ntest {\
            \n  test: bar(3, $c: true);\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  test: true;\
        \n}\
        \n"
    );
}
