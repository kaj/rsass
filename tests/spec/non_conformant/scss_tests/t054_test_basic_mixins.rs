//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/054_test_basic_mixins.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo {\
            \n  .foo {a: b}}\
            \n\
            \n@include foo;\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
