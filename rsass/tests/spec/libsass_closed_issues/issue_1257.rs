//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1257.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1257")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \n.foo {\
             \n  color: color.invert(red...);\
             \n}"),
        ".foo {\
         \n  color: aqua;\
         \n}\n"
    );
}
