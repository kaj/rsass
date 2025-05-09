//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/150_test_combinator_unification_tilde_plus.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("150_test_combinator_unification_tilde_plus")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a ~ x {a: b}\
             \n.b + y {@extend x}\n"),
        ".a ~ x, .a ~ .b + y, .a.b + y {\
         \n  a: b;\
         \n}\n"
    );
}
