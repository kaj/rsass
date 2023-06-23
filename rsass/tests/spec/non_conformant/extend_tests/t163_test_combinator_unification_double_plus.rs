//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/163_test_combinator_unification_double_plus.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("163_test_combinator_unification_double_plus")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a.a + x {a: b}\
             \nb.b + y {@extend x}\n"),
        "a.a + x {\
         \n  a: b;\
         \n}\n"
    );
}
