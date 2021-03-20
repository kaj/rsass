//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/176_test_combinator_unification_nested.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".a > .b + x {a: b}\
            \n.c > .d + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a > .b + x, .c.a > .d.b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}
