//! Tests auto-converted from "sass-spec/spec/css/selector/escaping.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("escaping")
}

#[test]
fn dollar_char() {
    assert_eq!(
        runner().ok(".u\\$ {a: b;}\n"),
        ".u\\$ {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
fn dollar_char_as_numeric() {
    assert_eq!(
        runner().ok(".u\\24 {a: b;}\n"),
        ".u\\$ {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn number_as_first_char_with_space() {
    assert_eq!(
        runner().ok(".\\31 u {a: b;}\n"),
        ".\\31 u {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn number_as_first_char_without_space() {
    assert_eq!(
        runner().ok(".\\31u {a: b;}\n"),
        ".\\31 u {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
fn number_as_nonfirst_char_with_space() {
    assert_eq!(
        runner().ok(".a\\31 u {a: b;}\n"),
        ".a1u {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
fn number_as_nonfirst_char_without_space() {
    assert_eq!(
        runner().ok(".a\\31u {a: b;}\n"),
        ".a1u {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
fn parenthesis_in_interpolation() {
    assert_eq!(
        runner().ok(".u#{\'\\\\28\'} { a: b; }\n"),
        ".u\\( {\
         \n  a: b;\
         \n}\n"
    );
}
