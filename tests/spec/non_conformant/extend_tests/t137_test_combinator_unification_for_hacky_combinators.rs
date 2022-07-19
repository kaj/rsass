//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/137_test_combinator_unification_for_hacky_combinators.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("137_test_combinator_unification_for_hacky_combinators")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".a + > x {a: b}\
             \n.b > + y {@extend x}\n"),
        ""
    );
}
