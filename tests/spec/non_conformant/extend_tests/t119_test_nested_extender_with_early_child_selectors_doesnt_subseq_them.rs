//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/119_test_nested_extender_with_early_child_selectors_doesnt_subseq_them.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            ".bap > .bip .foo {a: b}\
             \n.bap > .grip .bar {@extend .foo}\n"
        ),
        ".bap > .bip .foo, .bap > .bip .bap > .grip .bar, .bap > .grip .bap > .bip .bar {\
         \n  a: b;\
         \n}\n"
    );
}
