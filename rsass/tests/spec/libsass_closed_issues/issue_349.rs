//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_349.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_349")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  blah: not true;\
             \n}\n"),
        "div {\
         \n  blah: false;\
         \n}\n"
    );
}
