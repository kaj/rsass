//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/138_test_combinator_unification_for_hacky_combinators.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a + > x {a: b}\
             \n.b > + y {@extend x}\n"),
        ".a + > x {\
         \n  a: b;\
         \n}\n"
    );
}
