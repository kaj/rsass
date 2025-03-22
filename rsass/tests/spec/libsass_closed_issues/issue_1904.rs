//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1904.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1904")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  &--#{\'bar\'} {\
             \n    color: red;\
             \n  }\
             \n}"),
        ".foo--bar {\
         \n  color: red;\
         \n}\n"
    );
}
