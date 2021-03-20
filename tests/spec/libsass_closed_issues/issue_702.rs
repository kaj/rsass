//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_702.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  content: function-exists(\"feature-exists\");\
            \n  content: feature-exists(\"foo\");\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  content: true;\
        \n  content: false;\
        \n}\
        \n"
    );
}
