//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1107.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n    filter: progid:DXImageTransform.Microsoft.AlphaImageLoader(\
            \n        src=\"#{foo}\",\
            \n        sizingMethod=\'scale\');\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  filter: progid:DXImageTransform.Microsoft.AlphaImageLoader( src=\"foo\", sizingMethod=\"scale\");\
        \n}\
        \n"
    );
}
