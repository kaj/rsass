//! Tests auto-converted from "sass-spec/spec/mixin/content"
#[allow(unused)]
use super::rsass;

mod arguments;

// From "sass-spec/spec/mixin/content/before_if.hrx"
#[test]
fn before_if() {
    assert_eq!(
        rsass(
            "// Regression test for sass/dart-sass#482.\
            \n@mixin outer {\
            \n  a {@content}\
            \n}\
            \n\
            \n@mixin inner {\
            \n  @content;\
            \n}\
            \n\
            \n@include outer {\
            \n  @include inner {}\
            \n  x: y;\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  x: y;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/mixin/content/recursive.hrx"
#[test]
fn recursive() {
    assert_eq!(
        rsass(
            "@mixin span($i) {\
            \n  x: y;\
            \n  @content;\
            \n}\
            \n\
            \n.a {\
            \n  @include span(5) {\
            \n    .inner { @include span(2); }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".a {\
        \n  x: y;\
        \n}\
        \n.a .inner {\
        \n  x: y;\
        \n}\
        \n"
    );
}
