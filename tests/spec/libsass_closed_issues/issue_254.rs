//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_254.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin simple-media-query($max-width, $min-width) {\r\
            \n      @media only screen and (max-width: $max-width) and (min-width: $min-width) {\r\
            \n        @content;\r\
            \n      }\r\
            \n}\r\
            \n\r\
            \n@mixin test($value) {\r\
            \n    border-color: $value;\r\
            \n}\r\
            \n\r\
            \nbody \r\
            \n{\r\
            \n    @include test(\"#ccc\");\r\
            \n    @include simple-media-query(900px, 400px) {\r\
            \n        border-color: black;\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        "body {\
        \n  border-color: \"#ccc\";\
        \n}\
        \n@media only screen and (max-width: 900px) and (min-width: 400px) {\
        \n  body {\
        \n    border-color: black;\
        \n  }\
        \n}\
        \n"
    );
}
