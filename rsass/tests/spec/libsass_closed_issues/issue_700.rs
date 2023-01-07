//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_700.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_700")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".selector {\
             \n  color: invert(transparent);\
             \n}"),
        ".selector {\
         \n  color: rgba(255, 255, 255, 0);\
         \n}\n"
    );
}
