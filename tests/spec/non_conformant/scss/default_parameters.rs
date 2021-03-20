//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/default-parameters.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$a: red;\
            \n\
            \n@mixin f($a: $a) {\
            \n  color: $a;\
            \n}\
            \n\
            \nh1 {\
            \n  @include f;\
            \n}\
            \n\
            \nh2 {\
            \n  @include f(blue);\
            \n}"
        )
        .unwrap(),
        "h1 {\
        \n  color: red;\
        \n}\
        \nh2 {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
