//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/105_test_nested_extender_counts_extended_superselectors.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".a .bip .foo {a: b}\
            \n.b .bip.bop .bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".a .bip .foo, .a .b .bip.bop .bar, .b .a .bip.bop .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
