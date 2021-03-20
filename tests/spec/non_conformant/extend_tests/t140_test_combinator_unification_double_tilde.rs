//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/140_test_combinator_unification_double_tilde.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".a.b ~ x {a: b}\
            \n.a ~ y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a.b ~ x, .a.b ~ y {\
        \n  a: b;\
        \n}\
        \n"
    );
}
