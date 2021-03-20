//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/191_test_placeholder_selector_with_multiple_extenders.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%foo {color: blue}\
            \n.bar {@extend %foo}\
            \n.baz {@extend %foo}\
            \n"
        )
        .unwrap(),
        ".baz, .bar {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
