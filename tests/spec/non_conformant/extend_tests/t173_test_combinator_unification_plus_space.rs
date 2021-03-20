//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/173_test_combinator_unification_plus_space.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".a.b x {a: b}\
            \n.a + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a.b x, .a.b .a + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}
