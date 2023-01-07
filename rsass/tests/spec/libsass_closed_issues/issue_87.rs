//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_87.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_87")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$bar: \"bar\";\r\
             \n$foobar: \"foo#{$bar}\";\r\
             \n#{$bar} {\r\
             \n  #{$bar}: #{$bar};\r\
             \n  #{$bar}: $bar;\r\
             \n}\r\
             \n#{$foobar} {\r\
             \n  #{$foobar}: #{$foobar};\r\
             \n  #{$foobar}: $foobar;\r\
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
