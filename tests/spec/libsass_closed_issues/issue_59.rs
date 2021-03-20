//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_59.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin apply-to-ie6-only {\r\
            \n  * html {\r\
            \n    @content;\r\
            \n  }\r\
            \n}\r\
            \n@include apply-to-ie6-only {\r\
            \n  #logo {\r\
            \n    background-image: url(/logo.gif);\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        "* html #logo {\
        \n  background-image: url(/logo.gif);\
        \n}\
        \n"
    );
}
