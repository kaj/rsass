//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/139_test_combinator_unification_for_hacky_combinators.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a ~ > + .b > x {a: b}\
             \n.c > + .d > y {@extend x}\n"),
        ".a ~ > + .b > x, .a .c ~ > + .d.b > y, .c .a ~ > + .d.b > y {\
         \n  a: b;\
         \n}\n"
    );
}
