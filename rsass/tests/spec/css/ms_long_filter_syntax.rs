//! Tests auto-converted from "sass-spec/spec/css/ms_long_filter_syntax.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("ms_long_filter_syntax")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "foo {\
             \n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\
             \n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000); }\n"
        ),
        "foo {\
         \n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\
         \n  filter: progid:DXImageTransform.Microsoft.gradient(GradientType=1, startColorstr=#c0ff3300, endColorstr=#ff000000);\
         \n}\n"
    );
}
