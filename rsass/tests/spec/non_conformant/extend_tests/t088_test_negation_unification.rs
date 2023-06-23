//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/088_test_negation_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("088_test_negation_unification")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a :not(.foo).baz {a: b}\
             \n:not(.foo) {@extend .baz} -a {@extend %-a}\n"),
        "-a :not(.foo) {\
         \n  a: b;\
         \n}\n"
    );
}
