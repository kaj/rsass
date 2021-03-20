//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/basic_mixins.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo {a: b}\
            \n\
            \nbar {\
            \n  @include foo;\
            \n  c: d; }\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: b;\
        \n  c: d;\
        \n}\
        \n"
    );
}
