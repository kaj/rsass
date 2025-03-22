//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_201.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_201")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a, b, { color: red; }"),
        "a, b {\
         \n  color: red;\
         \n}\n"
    );
}
