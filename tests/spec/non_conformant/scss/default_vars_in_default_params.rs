//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/default-vars-in-default-params.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$y: why;\
            \n\
            \n@mixin foo($x, $y: $y) {\
            \n  stuff: $x $y;\
            \n}\
            \n\
            \ndiv {\
            \n  why: $y;\
            \n  @include foo(ecks);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  why: why;\
        \n  stuff: ecks why;\
        \n}\
        \n"
    );
}
