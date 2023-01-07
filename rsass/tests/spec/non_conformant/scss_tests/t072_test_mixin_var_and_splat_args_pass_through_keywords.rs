//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/072_test_mixin_var_and_splat_args_pass_through_keywords.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("072_test_mixin_var_and_splat_args_pass_through_keywords")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo($a...) {\
             \n  @include bar($a...);\
             \n}\n\
             \n@mixin bar($b, $c, $a) {\
             \n  a: $a;\
             \n  b: $b;\
             \n  c: $c;\
             \n}\n\
             \n.foo {@include foo(1, $c: 2, $a: 3)}\n"),
        ".foo {\
         \n  a: 3;\
         \n  b: 1;\
         \n  c: 2;\
         \n}\n"
    );
}
