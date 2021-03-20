//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/163_test_combinator_unification_double_plus.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "a.a + x {a: b}\
            \nb.b + y {@extend x}\
            \n"
        )
        .unwrap(),
        "a.a + x {\
        \n  a: b;\
        \n}\
        \n"
    );
}
