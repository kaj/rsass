//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/159_test_combinator_unification_double_angle.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a.a > x {a: b}\
             \nb.b > y {@extend x}\n"),
        "a.a > x {\
         \n  a: b;\
         \n}\n"
    );
}
