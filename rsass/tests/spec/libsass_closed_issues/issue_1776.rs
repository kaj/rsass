//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1776.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1776")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("h1 {\r\
             \n  width :calc(100% - 110px);\r\
             \n}"),
        "h1 {\
         \n  width: calc(100% - 110px);\
         \n}\n"
    );
}
