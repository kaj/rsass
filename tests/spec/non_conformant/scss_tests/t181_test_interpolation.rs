//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/181_test_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$bar : \"#foo\";\
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
