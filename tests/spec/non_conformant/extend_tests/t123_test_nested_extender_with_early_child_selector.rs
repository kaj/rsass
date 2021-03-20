//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/123_test_nested_extender_with_early_child_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n.bar {a: b}\
            \n.bip > .baz {@extend .bar}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo .bar, .foo .bip > .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
