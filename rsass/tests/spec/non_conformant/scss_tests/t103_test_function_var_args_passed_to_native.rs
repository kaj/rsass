//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/103_test_function_var_args_passed_to_native.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("103_test_function_var_args_passed_to_native")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \n@function foo($args...) {\
             \n  @return color.adjust($args...);\
             \n}\n\
             \n.foo {val: foo(#102030, $blue: 5)}\n"),
        ".foo {\
         \n  val: #102035;\
         \n}\n"
    );
}
