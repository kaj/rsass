//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1224.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1224")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@media all and (max-width: 768px) {\
             \n  @media only screen {\
             \n    a { b: c; }\
             \n  }\
             \n}\n"),
        "@media only screen and (max-width: 768px) {\
         \n  a {\
         \n    b: c;\
         \n  }\
         \n}\n"
    );
}
