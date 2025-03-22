//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_683.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_683")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "foo {\
             \n    filter: progid:DXImageTransform.Microsoft.AlphaImageLoader(src=\"data:image/png;base64,ABCD\",sizingMethod=crop);\
             \n}\n"
        ),
        "foo {\
         \n  filter: progid:DXImageTransform.Microsoft.AlphaImageLoader(src=\"data:image/png;base64,ABCD\",sizingMethod=crop);\
         \n}\n"
    );
}
