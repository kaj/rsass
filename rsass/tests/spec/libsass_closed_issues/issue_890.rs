//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_890.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_890")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  border: {\
             \n    right: 10px solid /*here is a comment*/;\
             \n  }\
             \n}\n"),
        ".foo {\
         \n  border-right: 10px solid;\
         \n}\n"
    );
}
