//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/101_test_nested_extender_unifies_common_substring.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".baz .bip .bap .bink .foo {a: b}\
            \n.brat .bip .bap bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".baz .bip .bap .bink .foo, .baz .brat .bip .bap .bink bar, .brat .baz .bip .bap .bink bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
