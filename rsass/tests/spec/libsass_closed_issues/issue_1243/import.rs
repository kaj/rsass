//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/import.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("import")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \"http://foo\"\n"),
        "@import \"http://foo\";\n"
    );
}
