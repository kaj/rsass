//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/debug.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("debug")
}

#[test]
fn test() {
    assert_eq!(runner().ok("@debug(\"foo\")\n"), "");
}
