//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/069_test_mixin_splat_args_with_var_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("069_test_mixin_splat_args_with_var_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo($a, $b...) {\
             \n  a: $a;\
             \n  b: $b;\
             \n}\n\
             \n$list: 2, 3, 4;\
             \n.foo {@include foo(1, $list...)}\n"),
        ".foo {\
         \n  a: 1;\
         \n  b: 2, 3, 4;\
         \n}\n"
    );
}
