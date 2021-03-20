//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/133_test_nested_function_def.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  @function foo() {@return 1}\
            \n  a: foo(); }\
            \n\
            \nbar {b: foo()}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 1;\
        \n}\
        \nbar {\
        \n  b: foo();\
        \n}\
        \n"
    );
}
