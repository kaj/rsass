//! Tests auto-converted from "sass-spec/spec/core_functions/string/length.hrx"

#[test]
fn combining_character() {
    assert_eq!(
        crate::rsass(
            "// Sass does *not* treat strings as sequences of glyphs, so this string which\
            \n// contains \"c\" followed by a combining umlaut should be considered two separate\
            \n// characters even though it\'s rendered as only one.\
            \na {b: str-length(\"c\\0308\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2;\
        \n}\
        \n"
    );
}
#[test]
fn double_width_character() {
    assert_eq!(
        crate::rsass(
            "// Sass treats strings as sequences of Unicode codepoint; it doesn\'t care if a\
            \n// character is represented as two UTF-16 code units.\
            \na {b: str-length(\"👭\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn empty() {
    assert_eq!(
        crate::rsass(
            "a {b: str-length(\"\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0;\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn multiple_characters() {
    assert_eq!(
        crate::rsass(
            "a {b: str-length(\"fblthp abatement\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 16;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: str-length($string: \"c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn one_character() {
    assert_eq!(
        crate::rsass(
            "a {b: str-length(\"c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn unquoted() {
    assert_eq!(
        crate::rsass(
            "a {b: str-length(loofamonster)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 12;\
        \n}\
        \n"
    );
}
