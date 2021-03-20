//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/098_test_nested_extender_runs_unification.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo.bar {a: b}\
            \nfoo bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo.bar, foo bar.bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
