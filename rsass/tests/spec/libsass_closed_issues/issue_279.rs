//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_279.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("issue_279")
        .mock_file("foo.scss", ".test-hello {\n  color: red;\n}\n\n.test-world {\n  @extend .test-hello;\n}\n")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".theme {\
             \n  @import \"foo.scss\";\
             \n}\n"),
        ".theme .test-hello, .theme .test-world {\
         \n  color: red;\
         \n}\n"
    );
}
