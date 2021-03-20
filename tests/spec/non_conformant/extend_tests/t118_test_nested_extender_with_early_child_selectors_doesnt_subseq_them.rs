//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/118_test_nested_extender_with_early_child_selectors_doesnt_subseq_them.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".bip > .bap .foo {a: b}\
            \n.grip > .bap .bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".bip > .bap .foo, .bip > .bap .grip > .bap .bar, .grip > .bap .bip > .bap .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
