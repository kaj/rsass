//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/105_test_nested_extender_counts_extended_superselectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("105_test_nested_extender_counts_extended_superselectors")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".a .bip .foo {a: b}\
             \n.b .bip.bop .bar {@extend .foo}\n"),
        ".a .bip .foo, .a .b .bip.bop .bar, .b .a .bip.bop .bar {\
         \n  a: b;\
         \n}\n"
    );
}
