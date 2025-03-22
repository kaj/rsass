//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/quotes-in-interpolated-strings.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("quotes-in-interpolated-strings")
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
