//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/mixin-content-selectors.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($x: 1) {\
            \n  foo-sel {\
            \n    @content;\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  $x: hey;\
            \n  @include foo() {\
            \n    bar {\
            \n      color: red;\
            \n      hux {\
            \n        msg: $x;\
            \n      }\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div foo-sel bar {\
        \n  color: red;\
        \n}\
        \ndiv foo-sel bar hux {\
        \n  msg: hey;\
        \n}\
        \n"
    );
}
