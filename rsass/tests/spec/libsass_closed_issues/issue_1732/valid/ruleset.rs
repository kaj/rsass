//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1732/valid/ruleset.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("ruleset")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo { \
             \n  color: green;\
             \n}"),
        "foo {\
         \n  color: green;\
         \n}\n"
    );
}
