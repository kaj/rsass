//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2150.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2150")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@media (min-width: 100px) {\
             \n  .parent > %child {\
             \n    color: blue;\
             \n  }\
             \n}\n\
             \n.foo {\
             \n  @extend %child;\
             \n}\n"),
        "@media (min-width: 100px) {\
         \n  .parent > .foo {\
         \n    color: blue;\
         \n  }\
         \n}\n"
    );
}
