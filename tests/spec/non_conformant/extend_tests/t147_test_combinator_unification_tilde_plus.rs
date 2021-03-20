//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/147_test_combinator_unification_tilde_plus.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "a.a + x {a: b}\
            \nb.b ~ y {@extend x}\
            \n"
        )
        .unwrap(),
        "a.a + x, b.b ~ a.a + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}
