//! Tests auto-converted from "sass-spec/spec/css/unicode_range/question_mark.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
