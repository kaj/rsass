//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/default-args.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($x: 1, $y: $x + 1) {\
            \n  value: $x, $y;\
            \n}\
            \n\
            \ndiv {\
            \n  @include foo();\
            \n  @include foo(2);\
            \n  @include foo($y: 3);\
            \n}\
            \n\
            \n$v: hey;\
            \n\
            \n@mixin bar($x: $v) {\
            \n  value: $x;\
            \n}\
            \n\
            \ndiv {\
            \n  $v: ho !global;\
            \n  @include bar();\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  value: 1, 2;\
        \n  value: 2, 3;\
        \n  value: 1, 3;\
        \n}\
        \ndiv {\
        \n  value: ho;\
        \n}\
        \n"
    );
}
