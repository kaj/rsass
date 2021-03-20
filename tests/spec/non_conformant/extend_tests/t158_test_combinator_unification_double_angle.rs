//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/158_test_combinator_unification_double_angle.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".a > x {a: b}\
            \n.b > y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a > x, .b.a > y {\
        \n  a: b;\
        \n}\
        \n"
    );
}
