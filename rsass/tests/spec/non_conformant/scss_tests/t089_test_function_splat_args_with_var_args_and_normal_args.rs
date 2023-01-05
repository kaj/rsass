//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/089_test_function_splat_args_with_var_args_and_normal_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "089_test_function_splat_args_with_var_args_and_normal_args",
    )
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($a, $b, $c...) {\
             \n  @return \"a: #{$a}, b: #{$b}, c: #{$c}\";\
             \n}\n\
             \n$list: 2, 3, 4;\
             \n.foo {val: foo(1, $list...)}\n"),
        ".foo {\
         \n  val: \"a: 1, b: 2, c: 3, 4\";\
         \n}\n"
    );
}
