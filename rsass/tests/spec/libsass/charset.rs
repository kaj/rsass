//! Tests auto-converted from "sass-spec/spec/libsass/charset.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("charset")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  content: to-upper-case(\"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ҈ݓ\");\
             \n}\n"),
        "@charset \"UTF-8\";\
         \ndiv {\
         \n  content: \"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ҈ݓ\";\
         \n}\n"
    );
}
