//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/143_test_combinator_unification_double_tilde.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "a.a ~ x {a: b}\
            \nb.b ~ y {@extend x}\
            \n"
        )
        .unwrap(),
        "a.a ~ x, a.a ~ b.b ~ y, b.b ~ a.a ~ y {\
        \n  a: b;\
        \n}\
        \n"
    );
}
