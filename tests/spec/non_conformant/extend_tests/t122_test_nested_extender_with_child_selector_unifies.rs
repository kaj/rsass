//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/122_test_nested_extender_with_child_selector_unifies.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n.bar {a: b}\
            \n> .baz {@extend .bar}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo .bar, .foo > .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
