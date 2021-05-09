//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/088_test_function_splat_args_with_var_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($a, $b...) {\
             \n  @return \"a: #{$a}, b: #{$b}\";\
             \n}\n\
             \n$list: 2, 3, 4;\
             \n.foo {val: foo(1, $list...)}\n"),
        ".foo {\
         \n  val: \"a: 1, b: 2, 3, 4\";\
         \n}\n"
    );
}
