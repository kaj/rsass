//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_112.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin media($var1, $var2) {\r\
            \n  @media screen and ($var1: $var2) {\r\
            \n    @content;\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@include media(max-device-width, 500px) {\r\
            \n  foo {\r\
            \n    bar: \"works\";\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        "@media screen and (max-device-width: 500px) {\
        \n  foo {\
        \n    bar: \"works\";\
        \n  }\
        \n}\
        \n"
    );
}
