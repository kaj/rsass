//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1797.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1797")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%not {\
             \n  color: red;\
             \n}\
             \n.not {\
             \n  @extend %not;\
             \n}\
             \ndiv:has(%not) {\
             \n  color: black;\
             \n}\n\
             \nbar {\
             \n  span:not(%not) {\
             \n    color: black;\
             \n  }\
             \n  span:not(&.foo) {\
             \n    color: black;\
             \n  }\
             \n  span:not(&%not) {\
             \n    color: black;\
             \n  }\
             \n}"),
        ".not {\
         \n  color: red;\
         \n}\
         \ndiv:has(.not) {\
         \n  color: black;\
         \n}\
         \nbar span:not(.not) {\
         \n  color: black;\
         \n}\
         \nspan:not(bar.foo) {\
         \n  color: black;\
         \n}\
         \nspan:not(bar.not) {\
         \n  color: black;\
         \n}\n"
    );
}
