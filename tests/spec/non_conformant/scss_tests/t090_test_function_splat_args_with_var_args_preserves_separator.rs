//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/090_test_function_splat_args_with_var_args_preserves_separator.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@function foo($a, $b...) {\
             \n  @return \"a: #{$a}, b: #{$b}\";\
             \n}\n\
             \n$list: 3 4 5;\
             \n.foo {val: foo(1, 2, $list...)}\n"),
        ".foo {\
         \n  val: \"a: 1, b: 2 3 4 5\";\
         \n}\n"
    );
}
