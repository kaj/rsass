//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/088_test_negation_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a :not(.foo).baz {a: b}\
             \n:not(.foo) {@extend .baz} -a {@extend %-a}\n"),
        "-a :not(.foo) {\
         \n  a: b;\
         \n}\n"
    );
}
