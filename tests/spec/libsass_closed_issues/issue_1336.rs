//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1336.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(runner().ok("@debug null;\n"), "");
}
