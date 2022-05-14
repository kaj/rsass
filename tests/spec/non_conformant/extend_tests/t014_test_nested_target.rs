//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/014_test_nested_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("014_test_nested_target")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo .bar {a: b}\
             \n.baz {@extend .bar}\n"),
        ".foo .bar, .foo .baz {\
         \n  a: b;\
         \n}\n"
    );
}
