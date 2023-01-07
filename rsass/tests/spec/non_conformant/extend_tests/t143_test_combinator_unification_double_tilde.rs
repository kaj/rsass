//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/143_test_combinator_unification_double_tilde.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("143_test_combinator_unification_double_tilde")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a.a ~ x {a: b}\
             \nb.b ~ y {@extend x}\n"),
        "a.a ~ x, a.a ~ b.b ~ y, b.b ~ a.a ~ y {\
         \n  a: b;\
         \n}\n"
    );
}
