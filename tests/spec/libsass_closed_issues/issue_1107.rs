//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1107.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1107")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            ".foo {\
             \n    filter: progid:DXImageTransform.Microsoft.AlphaImageLoader(\
             \n        src=\"#{foo}\",\
             \n        sizingMethod=\'scale\');\
             \n}\n"
        ),
        ".foo {\
         \n  filter: progid:DXImageTransform.Microsoft.AlphaImageLoader( src=\"foo\", sizingMethod=\"scale\");\
         \n}\n"
    );
}
