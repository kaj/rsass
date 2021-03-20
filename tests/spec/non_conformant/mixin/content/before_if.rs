//! Tests auto-converted from "sass-spec/spec/non_conformant/mixin/content/before_if.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
