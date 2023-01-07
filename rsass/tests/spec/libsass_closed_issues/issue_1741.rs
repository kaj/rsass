//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1741.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1741")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".header {\r\
             \n  .nav-text-link:not(&.popover-link) {\r\
             \n    margin: 10px;\r\
             \n  }\r\
             \n}"),
        ".nav-text-link:not(.header.popover-link) {\
         \n  margin: 10px;\
         \n}\n"
    );
}
