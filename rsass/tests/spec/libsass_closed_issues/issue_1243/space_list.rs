//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/space-list.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("space-list")
}

#[test]
fn test() {
    assert_eq!(runner().ok("$a: 1 2\n"), "");
}
