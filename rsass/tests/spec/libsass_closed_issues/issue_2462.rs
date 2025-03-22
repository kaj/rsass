//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2462.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2462")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \nb {\
             \n    color: color.adjust(Crimson, $lightness: 10%);\
             \n}\n"),
        "b {\
         \n  color: rgb(236.75, 54.25, 90.75);\
         \n}\n"
    );
}
