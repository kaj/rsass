//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/010_test_multiple_extends_with_multiple_extenders_and_single_target.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo.bar {a: b}\
            \n.baz {@extend .foo}\
            \n.bang {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo.bar, .foo.bang, .bar.baz, .baz.bang {\
        \n  a: b;\
        \n}\
        \n"
    );
}
