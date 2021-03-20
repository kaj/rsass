//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/182_test_nested_extend_loop.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".bar {\
            \na: b;\
            \n.foo {c: d; @extend .bar}\
            \n}\
            \n"
        )
        .unwrap(),
        ".bar, .bar .foo {\
        \n  a: b;\
        \n}\
        \n.bar .foo {\
        \n  c: d;\
        \n}\
        \n"
    );
}
