//! Tests auto-converted from "sass-spec/spec/css/unicode_range"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/css/unicode_range/error.hrx"
mod error {
    #[allow(unused)]
    use super::rsass;

    // Ignoring "ident_minus_space_ident", error tests are not supported yet.

    // Ignoring "minus_ident_minus", error tests are not supported yet.

    // Ignoring "minus_number_minus_ident", error tests are not supported yet.

    // Ignoring "no_digits", error tests are not supported yet.

    // Ignoring "nothing_after_minus", error tests are not supported yet.

    // Ignoring "question_mark_after_minus", error tests are not supported yet.

    // Ignoring "too_many_decimal_digits", error tests are not supported yet.

    // Ignoring "too_many_decimal_digits_after_minus", error tests are not supported yet.

    // Ignoring "too_many_digits_after_minus", error tests are not supported yet.

    // Ignoring "too_many_hex_digits", error tests are not supported yet.

    // Ignoring "too_many_question_marks", error tests are not supported yet.
}

// From "sass-spec/spec/css/unicode_range/question_mark.hrx"
#[test]
fn question_mark() {
    assert_eq!(
        rsass(
            ".question-mark {\
            \n  one-digit-question-only: U+?;\
            \n  four-digit-question-only: U+????;\
            \n  six-digit-question-only: U+??????;\
            \n  two-digit-half-question: U+A?;\
            \n  six-digit-half-question: U+123???;\
            \n\
            \n  // These are valid CSS, and should parse as a Unicode range followed by an\
            \n  // identifier.\
            \n  followed-by-ident: U+A?BCDE;\
            \n  followed-by-minus: U+A?-BCDE;\
            \n\
            \n  // This should parse as (U+A?) - (1234).\
            \n  followed-by-number: U+A?-1234;\
            \n}\
            \n"
        )
        .unwrap(),
        ".question-mark {\
        \n  one-digit-question-only: U+?;\
        \n  four-digit-question-only: U+????;\
        \n  six-digit-question-only: U+??????;\
        \n  two-digit-half-question: U+A?;\
        \n  six-digit-half-question: U+123???;\
        \n  followed-by-ident: U+A? BCDE;\
        \n  followed-by-minus: U+A? -BCDE;\
        \n  followed-by-number: U+A?-1234;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/unicode_range/range.hrx"
#[test]
fn range() {
    assert_eq!(
        rsass(
            ".range {\
            \n  one-digit-each: U+1-B;\
            \n  four-digit-each: U+1A2B-F9E8;\
            \n  six-digit-each: U+1A2B3C-10FFFF;\
            \n  one-then-six: U+1-000007;\
            \n  six-then-one: U+000001-7;\
            \n\
            \n  // Ruby and Dart Sass will allow ranges with values above the maximum allowed\
            \n  // code point, and ranges whose start values are greater than their end\
            \n  // values. These produce invalid CSS, though, and as such are not necessary\
            \n  // for all implementations to support.\
            \n}\
            \n"
        )
        .unwrap(),
        ".range {\
        \n  one-digit-each: U+1-B;\
        \n  four-digit-each: U+1A2B-F9E8;\
        \n  six-digit-each: U+1A2B3C-10FFFF;\
        \n  one-then-six: U+1-000007;\
        \n  six-then-one: U+000001-7;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/unicode_range/simple.hrx"
#[test]
fn simple() {
    assert_eq!(
        rsass(
            ".simple {\
            \n  one-digit: U+1;\
            \n  four-digit: U+1234;\
            \n  six-digit: U+123456;\
            \n  hex-digit: U+1A2B;\
            \n  lowercase: u+1a2b;\
            \n}\
            \n"
        )
        .unwrap(),
        ".simple {\
        \n  one-digit: U+1;\
        \n  four-digit: U+1234;\
        \n  six-digit: U+123456;\
        \n  hex-digit: U+1A2B;\
        \n  lowercase: u+1a2b;\
        \n}\
        \n"
    );
}
