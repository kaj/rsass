//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2321.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2321")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  b: if(true, b, c...);\
             \n  c: if(false, b, c...);\
             \n}\n"),
        "a {\
         \n  b: b;\
         \n  c: c;\
         \n}\n"
    );
}
