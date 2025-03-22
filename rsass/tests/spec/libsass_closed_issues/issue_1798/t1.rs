//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1798/1.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("1")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a /*#{\"}*/ {\
             \n  margin: 2px;\
             \n}\n"),
        "a {\
         \n  margin: 2px;\
         \n}\n"
    );
}
