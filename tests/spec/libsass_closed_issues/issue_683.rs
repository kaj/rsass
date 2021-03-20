//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_683.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n    filter: progid:DXImageTransform.Microsoft.AlphaImageLoader(src=\"data:image/png;base64,ABCD\",sizingMethod=crop);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  filter: progid:DXImageTransform.Microsoft.AlphaImageLoader(src=\"data:image/png;base64,ABCD\",sizingMethod=crop);\
        \n}\
        \n"
    );
}
