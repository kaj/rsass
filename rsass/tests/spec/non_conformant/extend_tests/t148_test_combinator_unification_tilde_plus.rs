//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/148_test_combinator_unification_tilde_plus.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("148_test_combinator_unification_tilde_plus")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a.b ~ x {a: b}\
             \n.a + y {@extend x}\n"),
        ".a.b ~ x, .a.b ~ .a + y, .a.b + y {\
         \n  a: b;\
         \n}\n"
    );
}
