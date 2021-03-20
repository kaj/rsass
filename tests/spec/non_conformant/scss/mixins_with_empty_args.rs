//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/mixins_with_empty_args.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo {a: b}\
            \n\
            \n.foo {@include foo();}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
