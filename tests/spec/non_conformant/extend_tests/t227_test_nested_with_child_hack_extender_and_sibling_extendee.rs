//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/227_test_nested_with_child_hack_extender_and_sibling_extendee.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "~ .foo {a: b}\
            \n> foo bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        "~ .foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
