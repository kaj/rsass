//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/085_test_function_var_args_act_like_list.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("085_test_function_var_args_act_like_list")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@function foo($a, $b...) {\
             \n  @return \"a: #{list.length($b)}, b: #{list.nth($b, 2)}\";\
             \n}\n\
             \n.foo {val: foo(1, 2, 3, 4)}\n"),
        ".foo {\
         \n  val: \"a: 3, b: 3\";\
         \n}\n"
    );
}
