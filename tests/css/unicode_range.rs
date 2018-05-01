//! Tests from `sass-spec/spec/css/unicode_range`

use super::check;

#[test]
fn simple() {
    check(
        ".simple {\n  one-digit: U+1;\n  four-digit: U+1234;\n  \
         six-digit: U+123456;\n  hex-digit: U+1A2B;\n  \
         lowercase: u+1a2b;\n}\n",
        ".simple {\n  one-digit: U+1;\n  four-digit: U+1234;\n  \
         six-digit: U+123456;\n  hex-digit: U+1A2B;\n  \
         lowercase: u+1a2b;\n}\n",
    )
}

#[test]
fn question_mark() {
    check(
        ".question-mark {\n  one-digit-question-only: U+?;\n  \
         four-digit-question-only: U+????;\n  \
         six-digit-question-only: U+??????;\n  \
         two-digit-half-question: U+A?;\n  \
         six-digit-half-question: U+123???;\n\n  \
         // These are valid CSS, and should parse as a Unicode \
         range followed by an\n  // identifier.\n  \
         followed-by-ident: U+A?BCDE;\n  \
         followed-by-minus: U+A?-BCDE;\n\n  \
         // This should parse as (U+A?) - (1234).\n  \
         followed-by-number: U+A?-1234;\n}\n",
        ".question-mark {\n  one-digit-question-only: U+?;\n  \
         four-digit-question-only: U+????;\n  \
         six-digit-question-only: U+??????;\n  \
         two-digit-half-question: U+A?;\n  \
         six-digit-half-question: U+123???;\n  \
         followed-by-ident: U+A? BCDE;\n  \
         followed-by-minus: U+A? -BCDE;\n  \
         followed-by-number: U+A?-1234;\n}\n",
    )
}

#[test]
fn range() {
    check(
        ".range {\n  one-digit-each: U+1-B;\n  \
         four-digit-each: U+1A2B-F9E8;\n  \
         six-digit-each: U+1A2B3C-10FFFF;\n  \
         one-then-six: U+1-000007;\n  six-then-one: U+000001-7;\n\n  \
         // Ruby and Dart Sass will allow ranges with values above \
         the maximum allowed\n  \
         // code point, and ranges whose start values are greater than \
         their end\n  \
         // values. These produce invalid CSS, though, and as such are \
         not necessary\n  \
         // for all implementations to support.\n}\n",
        ".range {\n  one-digit-each: U+1-B;\n  \
         four-digit-each: U+1A2B-F9E8;\n  \
         six-digit-each: U+1A2B3C-10FFFF;\n  \
         one-then-six: U+1-000007;\n  six-then-one: U+000001-7;\n}\n",
    )
}
