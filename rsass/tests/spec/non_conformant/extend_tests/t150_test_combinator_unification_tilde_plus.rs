//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/150_test_combinator_unification_tilde_plus.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("150_test_combinator_unification_tilde_plus")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a ~ x {a: b}\
             \n.b + y {@extend x}\n"),
        ".a ~ x, .a ~ .b + y, .b.a + y {\
         \n  a: b;\
         \n}\n"
    );
}
