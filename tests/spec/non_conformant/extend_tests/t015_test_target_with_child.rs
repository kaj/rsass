//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/015_test_target_with_child.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo .bar {a: b}\
            \n.baz {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo .bar, .baz .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
