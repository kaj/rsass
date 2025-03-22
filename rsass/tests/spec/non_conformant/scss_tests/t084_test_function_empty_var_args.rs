//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/084_test_function_empty_var_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("084_test_function_empty_var_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@function foo($a, $b...) {\
             \n  @return \"a: #{$a}, b: #{list.length($b)}\";\
             \n}\n\
             \n.foo {val: foo(1)}\n"),
        ".foo {\
         \n  val: \"a: 1, b: 0\";\
         \n}\n"
    );
}
