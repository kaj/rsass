//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2202.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2202")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@customAtRule;\r\
             \ntest {\r\
             \n  content: bar\r\
             \n}"),
        "@customAtRule;\
         \ntest {\
         \n  content: bar;\
         \n}\n"
    );
}
