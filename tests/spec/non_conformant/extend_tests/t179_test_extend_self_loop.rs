//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/179_test_extend_self_loop.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {a: b; @extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
