//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/091_test_function_var_and_splat_args_pass_through_keywords.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($a...) {\
             \n  @return bar($a...);\
             \n}\n\
             \n@function bar($b, $c, $a) {\
             \n  @return \"a: #{$a}, b: #{$b}, c: #{$c}\";\
             \n}\n\
             \n.foo {val: foo(1, $c: 2, $a: 3)}\n"),
        ".foo {\
         \n  val: \"a: 3, b: 1, c: 2\";\
         \n}\n"
    );
}
