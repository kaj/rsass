//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/087_test_function_splat_expression.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($a, $b, $c, $d) {\
             \n  @return \"a: #{$a}, b: #{$b}, c: #{$c}, d: #{$d}\";\
             \n}\n\
             \n.foo {val: foo(1, (2, 3, 4)...)}\n"),
        ".foo {\
         \n  val: \"a: 1, b: 2, c: 3, d: 4\";\
         \n}\n"
    );
}
