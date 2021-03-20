//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/061_test_mixins_with_args.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($a, $b) {\
            \n  a: $a;\
            \n  b: $b; }\
            \n\
            \n.foo {@include foo(bar, 12px)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: bar;\
        \n  b: 12px;\
        \n}\
        \n"
    );
}
