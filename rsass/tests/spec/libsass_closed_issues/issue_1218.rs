//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1218.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1218")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("$foo: 20px;\
             \n@media screen and (\"min-width:#{$foo}\") {\
             \n    .bar {\
             \n        width: 12px;\
             \n    }\
             \n}\
             \n@media screen and (\"min-width:0\") {\
             \n    .bar {\
             \n        width: 12px;\
             \n    }\
             \n}\n"),
        "@media screen and (min-width:20px) {\
         \n  .bar {\
         \n    width: 12px;\
         \n  }\
         \n}\
         \n@media screen and (min-width:0) {\
         \n  .bar {\
         \n    width: 12px;\
         \n  }\
         \n}\n"
    );
}
