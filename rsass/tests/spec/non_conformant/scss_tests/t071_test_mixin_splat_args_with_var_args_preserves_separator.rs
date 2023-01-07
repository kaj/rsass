//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/071_test_mixin_splat_args_with_var_args_preserves_separator.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "071_test_mixin_splat_args_with_var_args_preserves_separator",
    )
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@mixin foo($a, $b...) {\
             \n  a: $a;\
             \n  b: $b;\
             \n}\n\
             \n$list: 3 4 5;\
             \n.foo {@include foo(1, 2, $list...)}\n"),
        ".foo {\
         \n  a: 1;\
         \n  b: 2 3 4 5;\
         \n}\n"
    );
}
