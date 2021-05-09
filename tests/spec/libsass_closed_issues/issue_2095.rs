//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2095.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().mock_file("expected_output.scss", "")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@media all {\
             \n  @mixin foo() {}\
             \n}\n"),
        ""
    );
}
