//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_883.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_883")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  @foo {\
             \n    font: a;\
             \n  }\
             \n  @bar {\
             \n    color: b;\
             \n  }\
             \n}\n"),
        "@foo {\
         \n  div {\
         \n    font: a;\
         \n  }\
         \n}\
         \n@bar {\
         \n  div {\
         \n    color: b;\
         \n  }\
         \n}\n"
    );
}
