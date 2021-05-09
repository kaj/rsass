//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_block_directive_with_semicolon.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@foo {\
             \n  a: b; }\n\
             \n@bar {\
             \n  a: b; }\n"),
        "@foo {\
         \n  a: b;\
         \n}\
         \n@bar {\
         \n  a: b;\
         \n}\n"
    );
}
