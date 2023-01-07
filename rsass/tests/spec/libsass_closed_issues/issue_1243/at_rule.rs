//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/at-rule.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("at-rule")
}

#[test]
fn test() {
    assert_eq!(runner().ok("@foo bar\n"), "@foo bar;\n");
}
