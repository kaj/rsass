//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2149.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2149")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            ".foo {\
             \n  background: url(\'background.png\') -10px -10px/110% no-repeat\
             \n}\n"
        ),
        ".foo {\
         \n  background: url(\"background.png\") -10px -10px/110% no-repeat;\
         \n}\n"
    );
}
