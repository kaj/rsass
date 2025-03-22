//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/140_test_combinator_unification_double_tilde.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("140_test_combinator_unification_double_tilde")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a.b ~ x {a: b}\
             \n.a ~ y {@extend x}\n"),
        ".a.b ~ x, .a.b ~ y {\
         \n  a: b;\
         \n}\n"
    );
}
