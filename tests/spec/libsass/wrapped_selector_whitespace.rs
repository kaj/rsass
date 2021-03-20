//! Tests auto-converted from "sass-spec/spec/libsass/wrapped-selector-whitespace.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\r\
            \n  :-moz-any(a , b) {\r\
            \n    foo: foo;\r\
            \n  }\r\
            \n  :foo(a , b) {\r\
            \n    bar: bar;\r\
            \n  }\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "div :-moz-any(a, b) {\
        \n  foo: foo;\
        \n}\
        \ndiv :foo(a , b) {\
        \n  bar: bar;\
        \n}\
        \n"
    );
}
