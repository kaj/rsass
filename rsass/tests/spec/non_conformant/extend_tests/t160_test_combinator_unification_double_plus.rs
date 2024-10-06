//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/160_test_combinator_unification_double_plus.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("160_test_combinator_unification_double_plus")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a.b + x {a: b}\
             \n.b + y {@extend x}\n"),
        ".a.b + x, .a.b + y {\
         \n  a: b;\
         \n}\n"
    );
}
