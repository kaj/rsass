//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/183_test_multiple_extender_merges_with_superset_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {@extend .bar; @extend .baz}\
            \na.bar.baz {a: b}\
            \n"
        )
        .unwrap(),
        "a.bar.baz, a.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
