//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/139_test_combinator_unification_for_hacky_combinators.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".a ~ > + .b > x {a: b}\
            \n.c > + .d > y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a ~ > + .b > x, .a .c ~ > + .d.b > y, .c .a ~ > + .d.b > y {\
        \n  a: b;\
        \n}\
        \n"
    );
}
