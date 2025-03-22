//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_733.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_733")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function getter() {\
             \n  @return 42px;\
             \n}\n\
             \ntest {\
             \n  content: getter()-1;\
             \n  content: getter()- 1;\
             \n  content: getter() -1;\
             \n}\n"),
        "test {\
         \n  content: 41px;\
         \n  content: 41px;\
         \n  content: 42px -1;\
         \n}\n"
    );
}
