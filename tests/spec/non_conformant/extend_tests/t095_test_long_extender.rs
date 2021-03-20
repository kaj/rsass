//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/095_test_long_extender.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo.bar {a: b}\
            \n.baz.bang {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo.bar, .bar.baz.bang {\
        \n  a: b;\
        \n}\
        \n"
    );
}
