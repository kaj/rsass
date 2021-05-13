//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/087_test_negation_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a :not(.foo).baz {a: b}\
             \n:not(.bar) {@extend .baz} -a {@extend %-a}\n"),
        "-a :not(.foo).baz, -a :not(.foo):not(.bar) {\
         \n  a: b;\
         \n}\n"
    );
}
