//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1370.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin ico-common($imgUrl){\r\
            \n    display: inline-block;\r\
            \n    background: url(i/$imgUrl);\r\
            \n    background-repeat: no-repeat;\r\
            \n}\r\
            \n\r\
            \n@mixin ico-size($width,$height){\r\
            \n    width: $width;\r\
            \n    height: $height;\r\
            \n}\r\
            \n\r\
            \n.test{\r\
            \n    @include ico-common(\"icon.png\");\r\
            \n\r\
            \n    @include ico-size(100px, 100px);\r\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  display: inline-block;\
        \n  background: url(i/\"icon.png\");\
        \n  background-repeat: no-repeat;\
        \n  width: 100px;\
        \n  height: 100px;\
        \n}\
        \n"
    );
}
