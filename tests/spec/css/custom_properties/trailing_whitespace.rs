//! Tests auto-converted from "sass-spec/spec/css/custom_properties/trailing_whitespace.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".trailing-whitespace {\
            \n  --space: value ;\
            \n  --tab: value\t;\
            \n  --newline: value\
            \n;\
            \n  --before-closing-brace: value\
            \n}\
            \n"
        )
        .unwrap(),
        ".trailing-whitespace {\
        \n  --space: value ;\
        \n  --tab: value\t;\
        \n  --newline: value ;\
        \n  --before-closing-brace: value ;\
        \n}\
        \n"
    );
}
