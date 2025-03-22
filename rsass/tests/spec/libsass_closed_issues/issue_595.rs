//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_595.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_595")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n    color: red;\
             \n};\n"),
        "a {\
         \n  color: red;\
         \n}\n"
    );
}
