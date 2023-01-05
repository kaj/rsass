//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/warning.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("warning")
}

#[test]
fn test() {
    assert_eq!(runner().ok("@warning \"foo\"\n"), "@warning \"foo\";\n");
}
