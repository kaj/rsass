//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1419/unquoted.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unquoted")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  foo: to-upper-case(ab\\63 d);\
             \n}\n"),
        "foo {\
         \n  foo: ABCD;\
         \n}\n"
    );
}
