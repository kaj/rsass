//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1765.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1765")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "foo {\
             \n  bar: 20px /* height */ + 2*5px /* padding */ + 2*1px /*border*/;\
             \n}\n"
        ),
        "foo {\
         \n  bar: 32px;\
         \n}\n"
    );
}
