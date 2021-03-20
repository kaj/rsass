//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/003_test_basic.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {a: b}\
            \n.bar {c: d; @extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo, .bar {\
        \n  a: b;\
        \n}\
        \n.bar {\
        \n  c: d;\
        \n}\
        \n"
    );
}
