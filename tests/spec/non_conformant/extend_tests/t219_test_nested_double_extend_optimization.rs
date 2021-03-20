//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/219_test_nested_double_extend_optimization.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%foo %bar {\
            \na: b;\
            \n}\
            \n\
            \n.parent1 {\
            \n@extend %foo;\
            \n\
            \n.child {\
            \n  @extend %bar;\
            \n}\
            \n}\
            \n\
            \n.parent2 {\
            \n@extend %foo;\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent1 .child {\
        \n  a: b;\
        \n}\
        \n"
    );
}
