//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/comma-list.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comma-list")
}

#[test]
fn test() {
    assert_eq!(runner().ok("$a: 1, 2\n"), "");
}
