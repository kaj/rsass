//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/187_test_basic_placeholder_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%foo {a: b}\
            \n.bar {@extend %foo}\
            \n"
        )
        .unwrap(),
        ".bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
