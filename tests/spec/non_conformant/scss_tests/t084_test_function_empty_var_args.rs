//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/084_test_function_empty_var_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($a, $b...) {\
             \n  @return \"a: #{$a}, b: #{length($b)}\";\
             \n}\n\
             \n.foo {val: foo(1)}\n"),
        ".foo {\
         \n  val: \"a: 1, b: 0\";\
         \n}\n"
    );
}
