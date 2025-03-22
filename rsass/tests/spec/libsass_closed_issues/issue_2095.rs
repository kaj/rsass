//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2095.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("issue_2095")
        .mock_file("expected_output.scss", "")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@media all {\
             \n  @mixin foo() {}\
             \n}\n"),
        ""
    );
}
