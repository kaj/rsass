//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2233.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("issue_2233")
        .mock_file("foo.scss", "a { b: c; }\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@media all and (min-width: 100px) {\
             \n  @import \"foo\"\
             \n}\n"),
        "@media all and (min-width: 100px) {\
         \n  a {\
         \n    b: c;\
         \n  }\
         \n}\n"
    );
}
