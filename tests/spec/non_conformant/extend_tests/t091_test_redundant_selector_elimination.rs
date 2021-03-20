//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/091_test_redundant_selector_elimination.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo.bar {a: b}\
            \n.x {@extend .foo, .bar}\
            \n.y {@extend .foo, .bar}\
            \n"
        )
        .unwrap(),
        ".foo.bar, .y, .x {\
        \n  a: b;\
        \n}\
        \n"
    );
}
