//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/104_test_nested_extender_counts_extended_subselectors/104_test_nested_extender_counts_extended_subselectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("104_test_nested_extender_counts_extended_subselectors")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".a .bip.bop .foo {a: b}\
             \n.b .bip .bar {@extend .foo}\n"),
        ".a .bip.bop .foo, .a .b .bip.bop .bar, .b .a .bip.bop .bar {\
         \n  a: b;\
         \n}\n"
    );
}
