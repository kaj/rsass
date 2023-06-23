//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/230_test_extend_redundancy_elimination_when_it_would_reduce_specificity.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("230_test_extend_redundancy_elimination_when_it_would_reduce_specificity")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a {a: b}\
             \na.foo {@extend a}\n"),
        "a, a.foo {\
         \n  a: b;\
         \n}\n"
    );
}
