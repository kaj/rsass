//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/139_test_combinator_unification_for_hacky_combinators.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("139_test_combinator_unification_for_hacky_combinators")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a ~ > + .b > x {a: b}\
             \n.c > + .d > y {@extend x}\n"),
        ""
    );
}
