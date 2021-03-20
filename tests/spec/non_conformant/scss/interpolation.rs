//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$bar : \"#foo\";\
            \n\
            \n\
            \n\
            \nul li#{$bar} a span.label { foo: bar; }\
            \n"
        )
        .unwrap(),
        "ul li#foo a span.label {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}
