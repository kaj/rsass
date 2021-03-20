//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_87.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$bar: \"bar\";\r\
            \n$foobar: \"foo#{$bar}\";\r\
            \n#{$bar} {\r\
            \n  #{$bar}: #{$bar};\r\
            \n  #{$bar}: $bar;\r\
            \n}\r\
            \n#{$foobar} {\r\
            \n  #{$foobar}: #{$foobar};\r\
            \n  #{$foobar}: $foobar;\r\
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
