//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1766.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("issue_1766")
        .mock_file("foo.scss", "foo { bar: baz }\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@media all { @import \"foo.scss\" }\
             \n@media all { @import \"foo.scss\"; }\n"),
        "@media all {\
         \n  foo {\
         \n    bar: baz;\
         \n  }\
         \n}\
         \n@media all {\
         \n  foo {\
         \n    bar: baz;\
         \n  }\
         \n}\n"
    );
}
