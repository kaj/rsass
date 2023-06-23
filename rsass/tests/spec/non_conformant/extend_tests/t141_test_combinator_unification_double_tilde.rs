//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/141_test_combinator_unification_double_tilde.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("141_test_combinator_unification_double_tilde")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a ~ x {a: b}\
             \n.a.b ~ y {@extend x}\n"),
        ".a ~ x, .a.b ~ y {\
         \n  a: b;\
         \n}\n"
    );
}
