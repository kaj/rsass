//! Tests auto-converted from "sass-spec/spec/css/unicode_range/simple.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".simple {\
             \n  one-digit: U+1;\
             \n  four-digit: U+1234;\
             \n  six-digit: U+123456;\
             \n  hex-digit: U+1A2B;\
             \n  lowercase: u+1a2b;\
             \n}\n"),
        ".simple {\
         \n  one-digit: U+1;\
         \n  four-digit: U+1234;\
         \n  six-digit: U+123456;\
         \n  hex-digit: U+1A2B;\
         \n  lowercase: u+1a2b;\
         \n}\n"
    );
}
