//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/055_test_basic_mixins.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo {\
            \n  .foo {a: b}}\
            \n\
            \nbar {\
            \n  @include foo;\
            \n  c: d; }\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  c: d;\
        \n}\
        \nbar .foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
