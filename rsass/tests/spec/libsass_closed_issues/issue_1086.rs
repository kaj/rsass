//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1086.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1086")
}

#[test]
fn test() {
    assert_eq!(runner().ok("$map: (-1px: 12);"), "");
}
