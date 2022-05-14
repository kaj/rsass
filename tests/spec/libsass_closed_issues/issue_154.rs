//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_154.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_154")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("test {\r\
             \n  filter:alpha(opacity=75);\r\
             \n}"),
        "test {\
         \n  filter: alpha(opacity=75);\
         \n}\n"
    );
}
