//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/106_test_nested_extender_with_child_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".baz .foo {a: b}\
            \nfoo > bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".baz .foo, .baz foo > bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
