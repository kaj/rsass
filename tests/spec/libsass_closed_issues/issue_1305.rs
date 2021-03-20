//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1305.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n    content: call(\'unquote\', \'foo\', ()...);\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  content: foo;\
        \n}\
        \n"
    );
}
