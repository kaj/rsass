//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/126_test_nested_extender_with_early_child_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo + .bar {a: b}\
            \n.bip > .baz {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo + .bar, .bip > .foo + .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
