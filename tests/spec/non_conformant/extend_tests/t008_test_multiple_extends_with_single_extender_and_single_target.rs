//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/008_test_multiple_extends_with_single_extender_and_single_target.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo.bar {a: b}\
            \n.baz {@extend .foo; @extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo.bar, .baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
