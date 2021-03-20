//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/120_test_nested_extender_with_child_selector_unifies.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".baz.foo {a: b}\
            \nfoo > bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".baz.foo, foo > bar.baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
