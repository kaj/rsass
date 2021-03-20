//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/180_test_basic_extend_loop.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {a: b; @extend .bar}\
            \n.bar {c: d; @extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo, .bar {\
        \n  a: b;\
        \n}\
        \n.bar, .foo {\
        \n  c: d;\
        \n}\
        \n"
    );
}
