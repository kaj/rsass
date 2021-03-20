//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/29_if.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: a, b, 1+2;\
            \n\
            \n@if type-of(nth($x, 3)) == number {\
            \n  div {\
            \n    background: gray;\
            \n  }\
            \n}\
            \n\
            \n@if type-of(nth($x, 2)) == number {\
            \n  div {\
            \n    background: gray;\
            \n  }\
            \n}\
            \n@else if type-of(nth($x, 2)) == string {\
            \n  div {\
            \n    background: blue;\
            \n  }\
            \n}\
            \n\
            \n@if type-of(nth($x, 2)) == number {\
            \n  div {\
            \n    background: gray;\
            \n  }\
            \n}\
            \n@else if type-of(nth($x, 2)) == color {\
            \n  div {\
            \n    background: blue;\
            \n  }\
            \n}\
            \n@else {\
            \n  div {\
            \n    background: red;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  background: gray;\
        \n}\
        \ndiv {\
        \n  background: blue;\
        \n}\
        \ndiv {\
        \n  background: red;\
        \n}\
        \n"
    );
}
