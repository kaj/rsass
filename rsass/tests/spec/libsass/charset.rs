//! Tests auto-converted from "sass-spec/spec/libsass/charset.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("charset")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \ndiv {\
             \n  content: string.to-upper-case(\"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ\u{488}ݓ\");\
             \n}\n"
        ),
        "@charset \"UTF-8\";\
         \ndiv {\
         \n  content: \"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ\u{488}ݓ\";\
         \n}\n"
    );
}
