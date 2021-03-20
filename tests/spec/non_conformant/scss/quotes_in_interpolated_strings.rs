//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/quotes-in-interpolated-strings.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$bar: \"bar\";\
            \n$foobar: \"foo#{$bar}\";\
            \n#{$bar} {\
            \n  #{$bar}: #{$bar};\
            \n  #{$bar}: $bar;\
            \n}\
            \nfoobar {\
            \n  #{$foobar}: #{$foobar};\
            \n  #{$foobar}: $foobar;\
            \n}"
        )
        .unwrap(),
        "bar {\
        \n  bar: bar;\
        \n  bar: \"bar\";\
        \n}\
        \nfoobar {\
        \n  foobar: foobar;\
        \n  foobar: \"foobar\";\
        \n}\
        \n"
    );
}
