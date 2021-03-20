//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/103_test_function_var_args_passed_to_native.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo($args...) {\
            \n  @return adjust-color($args...);\
            \n}\
            \n\
            \n.foo {val: foo(#102030, $blue: 5)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: #102035;\
        \n}\
        \n"
    );
}
