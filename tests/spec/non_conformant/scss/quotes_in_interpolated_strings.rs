//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/quotes-in-interpolated-strings.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$bar: \"bar\";\
             \n$foobar: \"foo#{$bar}\";\
             \n#{$bar} {\
             \n  #{$bar}: #{$bar};\
             \n  #{$bar}: $bar;\
             \n}\
             \nfoobar {\
             \n  #{$foobar}: #{$foobar};\
             \n  #{$foobar}: $foobar;\
             \n}"),
        "bar {\
         \n  bar: bar;\
         \n  bar: \"bar\";\
         \n}\
         \nfoobar {\
         \n  foobar: foobar;\
         \n  foobar: \"foobar\";\
         \n}\n"
    );
}
