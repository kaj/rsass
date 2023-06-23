//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/089_test_negation_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("089_test_negation_unification")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a :not([a=b]).baz {a: b}\
             \n:not([a = b]) {@extend .baz} -a {@extend %-a}\n"),
        "-a :not([a=b]) {\
         \n  a: b;\
         \n}\n"
    );
}
