//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_738.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_738")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  &--bar { color: red; }\
             \n  &--1bar { color: blue;}\
             \n}\n"),
        ".foo--bar {\
         \n  color: red;\
         \n}\
         \n.foo--1bar {\
         \n  color: blue;\
         \n}\n"
    );
}
