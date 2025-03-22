//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_453.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_453")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n    --a: 2px;\
             \n    top: var(--a);\
             \n}\n"),
        "div {\
         \n  --a: 2px;\
         \n  top: var(--a);\
         \n}\n"
    );
}
