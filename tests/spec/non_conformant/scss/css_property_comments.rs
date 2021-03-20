//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_property_comments.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  /* Foo\
            \n   * Bar */\
            \n  a: b; }\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  /* Foo\
        \n   * Bar */\
        \n  a: b;\
        \n}\
        \n"
    );
}
