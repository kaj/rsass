//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/231_test_extend_redundancy_elimination_when_it_would_preserve_specificity.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("231_test_extend_redundancy_elimination_when_it_would_preserve_specificity")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".bar a {a: b}\
             \na.foo {@extend a}\n"),
        ".bar a {\
         \n  a: b;\
         \n}\n"
    );
}
