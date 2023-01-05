//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1394.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1394")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  width: \\10 + \\20 \\ ;\
             \n}\n"),
        "foo {\
         \n  width: \\10 \\ \\ ;\
         \n}\n"
    );
}
