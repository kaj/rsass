//! Tests auto-converted from "sass-spec/spec/css/custom_properties/without_semicolon.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".simple-value {\
            \n  // A custom property at the end of a style rule doesn\'t need a semicolon.\
            \n  --without-semicolon: value\
            \n}\
            \n\
            \n.bracketed-value {\
            \n  --without-semicolon: {\
            \n    a: b\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".simple-value {\
        \n  --without-semicolon: value ;\
        \n}\
        \n.bracketed-value {\
        \n  --without-semicolon: {\
        \n    a: b\
        \n  } ;\
        \n}\
        \n"
    );
}
