//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/170_test_combinator_unification_plus_space.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("170_test_combinator_unification_plus_space")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a.b + x {a: b}\
             \n.a y {@extend x}\n"),
        ".a.b + x, .a .a.b + y {\
         \n  a: b;\
         \n}\n"
    );
}
