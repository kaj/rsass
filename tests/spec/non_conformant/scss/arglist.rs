//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/arglist.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($x, $y, $zs...) {\
            \n  foo-x: $x;\
            \n  foo-y: $y;\
            \n  foo-zs: $zs;\
            \n}\
            \n\
            \ndiv {\
            \n  @include foo(a, b, c, d, e);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo-x: a;\
        \n  foo-y: b;\
        \n  foo-zs: c, d, e;\
        \n}\
        \n"
    );
}
