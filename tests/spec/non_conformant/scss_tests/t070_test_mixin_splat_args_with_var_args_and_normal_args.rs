//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/070_test_mixin_splat_args_with_var_args_and_normal_args.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($a, $b, $c...) {\
            \n  a: $a;\
            \n  b: $b;\
            \n  c: $c;\
            \n}\
            \n\
            \n$list: 2, 3, 4;\
            \n.foo {@include foo(1, $list...)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 1;\
        \n  b: 2;\
        \n  c: 3, 4;\
        \n}\
        \n"
    );
}
