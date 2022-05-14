//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1839.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1839")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@custom-media --large-viewport (min-width: 1001px);"),
        "@custom-media --large-viewport (min-width: 1001px);\n"
    );
}
