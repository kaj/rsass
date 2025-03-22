//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2468.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2468")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%matches {\
             \n  :matches(oh, no) {\
             \n      x: 1;\
             \n      y: 2;\
             \n  }\
             \n}\
             \nmatches {\
             \n  @extend %matches;\
             \n  @extend oh;\
             \n}\n\
             \n%any {\
             \n  :any(oh, no) {\
             \n      x: 1;\
             \n      y: 2;\
             \n  }\
             \n}\
             \nany {\
             \n  @extend %any;\
             \n  @extend oh;\
             \n}\n"),
        "matches :matches(oh, any, matches, no) {\
         \n  x: 1;\
         \n  y: 2;\
         \n}\
         \nany :any(oh, any, matches, no) {\
         \n  x: 1;\
         \n  y: 2;\
         \n}\n"
    );
}
