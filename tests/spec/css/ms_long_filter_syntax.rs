//! Tests auto-converted from "sass-spec/spec/css/ms_long_filter_syntax.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\
            \n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000); }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\
        \n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\
        \n}\
        \n"
    );
}
