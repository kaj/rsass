//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1061.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1061")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  &.div,\
             \n  &.span {\
             \n    display: block;\
             \n  }\
             \n}\n"),
        "a.div, a.span {\
         \n  display: block;\
         \n}\n"
    );
}
