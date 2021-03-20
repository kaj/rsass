//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/072_test_mixin_var_and_splat_args_pass_through_keywords.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($a...) {\
            \n  @include bar($a...);\
            \n}\
            \n\
            \n@mixin bar($b, $c, $a) {\
            \n  a: $a;\
            \n  b: $b;\
            \n  c: $c;\
            \n}\
            \n\
            \n.foo {@include foo(1, $c: 2, $a: 3)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 3;\
        \n  b: 1;\
        \n  c: 2;\
        \n}\
        \n"
    );
}
