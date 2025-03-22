//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1732/valid/propset.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("propset")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo { \
             \n  border: {\
             \n    width: 1px;\
             \n    color: green;\
             \n  }\
             \n}"),
        "foo {\
         \n  border-width: 1px;\
         \n  border-color: green;\
         \n}\n"
    );
}
