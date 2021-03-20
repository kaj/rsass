//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/159_test_combinator_unification_double_angle.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "a.a > x {a: b}\
            \nb.b > y {@extend x}\
            \n"
        )
        .unwrap(),
        "a.a > x {\
        \n  a: b;\
        \n}\
        \n"
    );
}
