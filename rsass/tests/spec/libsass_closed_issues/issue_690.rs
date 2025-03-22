//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_690.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_690")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("test {\
             \n  left: expression(callSomeFunc());\
             \n  content: expression(\"Smile :-)\");\
             \n}\n"),
        "test {\
         \n  left: expression(callSomeFunc());\
         \n  content: expression(\"Smile :-)\");\
         \n}\n"
    );
}
