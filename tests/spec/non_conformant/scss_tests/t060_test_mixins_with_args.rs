//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/060_test_mixins_with_args.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($a) {a: $a}\
            \n\
            \n.foo {@include foo(bar)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: bar;\
        \n}\
        \n"
    );
}
