//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/104_test_nested_extender_counts_extended_subselectors/104_test_nested_extender_counts_extended_subselectors.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".a .bip.bop .foo {a: b}\
            \n.b .bip .bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".a .bip.bop .foo, .a .b .bip.bop .bar, .b .a .bip.bop .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
