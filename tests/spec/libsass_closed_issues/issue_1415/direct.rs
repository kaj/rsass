//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1415/direct.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@if & {\
             \n  foo {\
             \n    foo: bar;\
             \n  }\
             \n}\n"),
        ""
    );
}
