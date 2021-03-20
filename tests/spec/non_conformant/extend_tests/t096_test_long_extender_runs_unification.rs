//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/096_test_long_extender_runs_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "ns|*.foo.bar {a: b}\
            \na.baz {@extend .foo}\
            \n"
        )
        .unwrap(),
        "ns|*.foo.bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
