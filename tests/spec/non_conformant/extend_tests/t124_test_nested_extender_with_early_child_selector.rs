//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/124_test_nested_extender_with_early_child_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n.bip .bar {a: b}\
            \n> .baz {@extend .bar}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo .bip .bar, .foo .bip .foo > .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
