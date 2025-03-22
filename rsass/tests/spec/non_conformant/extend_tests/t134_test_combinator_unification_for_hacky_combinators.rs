//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/134_test_combinator_unification_for_hacky_combinators.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("134_test_combinator_unification_for_hacky_combinators")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a x {a: b}\
             \n.b > + y {@extend x}\n"),
        ".a x {\
         \n  a: b;\
         \n}\n"
    );
}
