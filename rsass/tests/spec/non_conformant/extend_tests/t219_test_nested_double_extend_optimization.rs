//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/219_test_nested_double_extend_optimization.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("219_test_nested_double_extend_optimization")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%foo %bar {\
             \na: b;\
             \n}\n\
             \n.parent1 {\
             \n@extend %foo;\n\
             \n.child {\
             \n  @extend %bar;\
             \n}\
             \n}\n\
             \n.parent2 {\
             \n@extend %foo;\
             \n}\n"),
        ".parent1 .child {\
         \n  a: b;\
         \n}\n"
    );
}
