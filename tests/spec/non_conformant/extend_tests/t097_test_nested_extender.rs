//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/097_test_nested_extender.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {a: b}\
            \nfoo bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo, foo bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
