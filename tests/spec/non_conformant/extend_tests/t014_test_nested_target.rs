//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/014_test_nested_target.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo .bar {a: b}\
            \n.baz {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo .bar, .foo .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
