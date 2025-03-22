//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/151_test_combinator_unification_tilde_plus.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("151_test_combinator_unification_tilde_plus")
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
