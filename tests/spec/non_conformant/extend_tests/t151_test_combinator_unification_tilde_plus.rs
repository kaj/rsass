//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/151_test_combinator_unification_tilde_plus.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a.a ~ x {a: b}\
             \nb.b + y {@extend x}\n"),
        "a.a ~ x, a.a ~ b.b + y {\
         \n  a: b;\
         \n}\n"
    );
}
