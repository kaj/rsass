//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_623.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_623")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  filter: alpha(opacity=.3); }\n\
             \ndiv {\
             \n  filter: alpha(opacity=0.7); }\n"),
        "a {\
         \n  filter: alpha(opacity=0.3);\
         \n}\
         \ndiv {\
         \n  filter: alpha(opacity=0.7);\
         \n}\n"
    );
}
