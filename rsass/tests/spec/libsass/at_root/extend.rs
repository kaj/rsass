//! Tests auto-converted from "sass-spec/spec/libsass/at-root/extend.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("extend")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  @at-root {\
             \n    %placeholder {\
             \n      color: red;\
             \n    }\
             \n  }\n\
             \n  baz {\
             \n    @at-root {\
             \n      %other-placeholder {\
             \n        color: blue;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \nbar {\
             \n  @extend %placeholder;\
             \n}\n\
             \nbaz {\
             \n  @extend %other-placeholder;\
             \n}\n"),
        "bar {\
         \n  color: red;\
         \n}\
         \nbaz {\
         \n  color: blue;\
         \n}\n"
    );
}
