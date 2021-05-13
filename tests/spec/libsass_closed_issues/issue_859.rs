//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_859.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@media screen {\
             \n  .two {\
             \n    @at-root .one {\
             \n      background: blue;\
             \n      .three {\
             \n        color: red;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n"),
        "@media screen {\
         \n  .one {\
         \n    background: blue;\
         \n  }\
         \n  .one .three {\
         \n    color: red;\
         \n  }\
         \n}\n"
    );
}
