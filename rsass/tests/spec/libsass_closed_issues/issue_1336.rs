//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1336.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1336")
}

#[test]
fn test() {
    assert_eq!(runner().ok("@debug null;\n"), "");
}
