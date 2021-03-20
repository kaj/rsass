//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/100_test_nested_extender_unifies_identical_parents.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".baz .bip .foo {a: b}\
            \n.baz .bip bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".baz .bip .foo, .baz .bip bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
