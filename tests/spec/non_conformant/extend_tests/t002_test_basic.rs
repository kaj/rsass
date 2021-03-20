//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/002_test_basic.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".bar {@extend .foo}\
            \n.foo {a: b}\
            \n"
        )
        .unwrap(),
        ".foo, .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
