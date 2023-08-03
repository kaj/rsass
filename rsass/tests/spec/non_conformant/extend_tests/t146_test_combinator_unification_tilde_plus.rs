//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/146_test_combinator_unification_tilde_plus.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("146_test_combinator_unification_tilde_plus")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a + x {a: b}\
             \n.b ~ y {@extend x}\n"),
        ".a + x, .b ~ .a + y, .a.b + y {\
         \n  a: b;\
         \n}\n"
    );
}
