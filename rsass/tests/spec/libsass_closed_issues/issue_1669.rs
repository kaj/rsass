//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1669.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1669")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar: #{100%/3}\
             \n}\n\n"),
        "foo {\
         \n  bar: 100%/3;\
         \n}\n"
    );
}
