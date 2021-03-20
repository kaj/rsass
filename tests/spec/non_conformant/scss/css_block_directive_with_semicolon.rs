//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_block_directive_with_semicolon.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@foo {\
            \n  a: b; }\
            \n\
            \n@bar {\
            \n  a: b; }\
            \n"
        )
        .unwrap(),
        "@foo {\
        \n  a: b;\
        \n}\
        \n@bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
