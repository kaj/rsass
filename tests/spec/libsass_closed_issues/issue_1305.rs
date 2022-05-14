//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1305.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1305")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n    content: call(\'unquote\', \'foo\', ()...);\
             \n}\n"),
        ".foo {\
         \n  content: foo;\
         \n}\n"
    );
}
