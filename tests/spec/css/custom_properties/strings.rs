//! Tests auto-converted from "sass-spec/spec/css/custom_properties/strings.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".strings {\
            \n  // Strings are tokenized as units, so their contents shouldn\'t affect anything\
            \n  // else.\
            \n  --text: \"foo\";\
            \n  --bang: \"!\";\
            \n  --semicolon: \";\";\
            \n  --square: \"][\";\
            \n  --curly: \"}{\";\
            \n  --parens: \")(\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".strings {\
        \n  --text: \"foo\";\
        \n  --bang: \"!\";\
        \n  --semicolon: \";\";\
        \n  --square: \"][\";\
        \n  --curly: \"}{\";\
        \n  --parens: \")(\";\
        \n}\
        \n"
    );
}
