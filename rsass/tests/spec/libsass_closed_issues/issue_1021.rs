//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1021.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1021")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\r\
             \n    top: 10px - 2 * 5px /* arrow size */;\r\
             \n}"),
        "div {\
         \n  top: 0px;\
         \n}\n"
    );
}
