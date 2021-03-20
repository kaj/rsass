//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/134_test_nested_function_shadow.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo() {@return 1}\
            \n\
            \nfoo {\
            \n  @function foo() {@return 2}\
            \n  a: foo();\
            \n}\
            \n\
            \nbaz {b: foo()}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 2;\
        \n}\
        \nbaz {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
