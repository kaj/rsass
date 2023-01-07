//! Tests auto-converted from "sass-spec/spec/css/custom_properties/strings.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("strings")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            ".strings {\
             \n  // Strings are tokenized as units, so their contents shouldn\'t affect anything\
             \n  // else.\
             \n  --text: \"foo\";\
             \n  --bang: \"!\";\
             \n  --semicolon: \";\";\
             \n  --square: \"][\";\
             \n  --curly: \"}{\";\
             \n  --parens: \")(\";\
             \n}\n"
        ),
        ".strings {\
         \n  --text: \"foo\";\
         \n  --bang: \"!\";\
         \n  --semicolon: \";\";\
         \n  --square: \"][\";\
         \n  --curly: \"}{\";\
         \n  --parens: \")(\";\
         \n}\n"
    );
}
