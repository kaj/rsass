//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/121_test_nested_extender_with_child_selector_unifies.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".baz > {\
            \n.foo {a: b}\
            \n.bar {@extend .foo}\
            \n}\
            \n"
        )
        .unwrap(),
        ".baz > .foo, .baz > .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
