//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/132_test_nested_extender_with_child_selector_merges_with_same_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo > .bar .baz {a: b}\
            \n.foo > .bar .bang {@extend .baz}\
            \n"
        )
        .unwrap(),
        ".foo > .bar .baz, .foo > .bar .bang {\
        \n  a: b;\
        \n}\
        \n"
    );
}
