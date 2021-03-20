//! Tests auto-converted from "sass-spec/spec/css/unicode_range/range.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
