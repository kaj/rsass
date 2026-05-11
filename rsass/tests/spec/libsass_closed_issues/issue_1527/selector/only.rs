//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1527/selector/only.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("only")
}

#[test]
fn test() {
    assert_eq!(runner().ok("& {}\n"), "");
}
