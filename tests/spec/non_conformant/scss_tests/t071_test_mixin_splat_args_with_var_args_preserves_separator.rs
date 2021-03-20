//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/071_test_mixin_splat_args_with_var_args_preserves_separator.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($a, $b...) {\
            \n  a: $a;\
            \n  b: $b;\
            \n}\
            \n\
            \n$list: 3 4 5;\
            \n.foo {@include foo(1, 2, $list...)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 1;\
        \n  b: 2 3 4 5;\
        \n}\
        \n"
    );
}
