//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/import.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \"http://foo\"\n"),
        "@import \"http://foo\";\n"
    );
}
