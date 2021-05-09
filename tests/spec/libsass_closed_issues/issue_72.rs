//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_72.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            "test {\r\
             \n  filter: progid:DXImageTransform.Microsoft.gradient( startColorstr=\'#223344\', endColorstr=\'#112233\',GradientType=0 );\r\
             \n}\r\
             \n\r\
             \n@mixin opacity($opacity) {\r\
             \n    opacity: $opacity / 100;\r\
             \n    filter: alpha(opacity=$opacity);\r\
             \n}"
        ),
        "test {\
         \n  filter: progid:DXImageTransform.Microsoft.gradient( startColorstr=\"#223344\", endColorstr=\"#112233\",GradientType=0 );\
         \n}\n"
    );
}
