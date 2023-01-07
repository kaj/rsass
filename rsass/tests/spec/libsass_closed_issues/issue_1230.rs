//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1230.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1230")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  transition-property:\
             \n    border-color,\
             \n    box-shadow,\
             \n    color;\
             \n}"),
        "div {\
         \n  transition-property: border-color, box-shadow, color;\
         \n}\n"
    );
}
