//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1732/valid/mixin-def.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin-def")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin a {\
             \n  b: c;\
             \n}\n"),
        ""
    );
}
