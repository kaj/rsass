//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/064_test_mixin_var_args.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($a, $b...) {\
            \n  a: $a;\
            \n  b: $b;\
            \n}\
            \n\
            \n.foo {@include foo(1, 2, 3, 4)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 1;\
        \n  b: 2, 3, 4;\
        \n}\
        \n"
    );
}
