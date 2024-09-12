//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/066_test_mixin_var_args_act_like_list.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("066_test_mixin_var_args_act_like_list")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@mixin foo($a, $b...) {\
             \n  a: list.length($b);\
             \n  b: list.nth($b, 2);\
             \n}\n\
             \n.foo {@include foo(1, 2, 3, 4)}\n"),
        ".foo {\
         \n  a: 3;\
         \n  b: 3;\
         \n}\n"
    );
}
