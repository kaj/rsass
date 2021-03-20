//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_72.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\r\
            \n  filter: progid:DXImageTransform.Microsoft.gradient( startColorstr=\'#223344\', endColorstr=\'#112233\',GradientType=0 );\r\
            \n}\r\
            \n\r\
            \n@mixin opacity($opacity) {\r\
            \n    opacity: $opacity / 100;\r\
            \n    filter: alpha(opacity=$opacity);\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  filter: progid:DXImageTransform.Microsoft.gradient( startColorstr=\"#223344\", endColorstr=\"#112233\",GradientType=0 );\
        \n}\
        \n"
    );
}
