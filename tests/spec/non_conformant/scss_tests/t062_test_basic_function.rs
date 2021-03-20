//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/062_test_basic_function.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo() {\
            \n  @return 1 + 2;\
            \n}\
            \n\
            \nbar {\
            \n  a: foo();\
            \n}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: 3;\
        \n}\
        \n"
    );
}
