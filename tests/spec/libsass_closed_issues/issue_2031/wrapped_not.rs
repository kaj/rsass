//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2031/wrapped-not.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("wrapped-not")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(":not(.asd, .qwe) {\r\
             \n  content: test;\r\
             \n}"),
        ":not(.asd, .qwe) {\
         \n  content: test;\
         \n}\n"
    );
}
