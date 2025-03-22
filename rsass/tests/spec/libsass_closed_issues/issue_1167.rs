//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1167.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1167")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  b: 3s + 101ms;\
             \n}"),
        "a {\
         \n  b: 3.101s;\
         \n}\n"
    );
}
