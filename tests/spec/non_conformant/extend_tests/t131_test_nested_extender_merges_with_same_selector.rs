//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/131_test_nested_extender_merges_with_same_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n.bar {a: b}\
            \n.baz {@extend .bar} }\
            \n"
        )
        .unwrap(),
        ".foo .bar, .foo .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
