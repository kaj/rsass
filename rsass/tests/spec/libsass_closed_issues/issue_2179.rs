//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2179.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2179")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@mixin sprite-arrow() {\
             \n  @extend %hidden-text;\
             \n}\n\
             \n%hidden-text {\
             \n  text-indent: -999em;\
             \n}\n\
             \n// button.scss\
             \n.button-left,\
             \n.button-right,\
             \n.button-plus,\
             \n.button-min {\
             \n  &:after {\
             \n      @include sprite-arrow();\
             \n  }\
             \n}\n\
             \n.banner {\
             \n  &:after {\
             \n      @include sprite-arrow();\
             \n  }\
             \n}\n\
             \n.calculator {\
             \n  .btn-down,\
             \n  .btn-up {\
             \n      &:after {\
             \n          @include sprite-arrow();\
             \n      }\
             \n  }\
             \n}"),
        ".calculator .btn-down:after,\
         \n.calculator .btn-up:after, .banner:after, .button-left:after,\
         \n.button-right:after,\
         \n.button-plus:after,\
         \n.button-min:after {\
         \n  text-indent: -999em;\
         \n}\n"
    );
}
