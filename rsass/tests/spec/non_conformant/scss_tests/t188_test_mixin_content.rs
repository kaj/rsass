//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/188_test_mixin_content.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("188_test_mixin_content")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$color: blue;\
             \n@mixin context($class, $color: red) {\
             \n  .#{$class} {\
             \n    background-color: $color;\
             \n    @content;\
             \n    border-color: $color;\
             \n  }\
             \n}\
             \n@include context(parent) {\
             \n  @include context(child, $color: yellow) {\
             \n    color: $color;\
             \n  }\
             \n}\n"),
        ".parent {\
         \n  background-color: red;\
         \n}\
         \n.parent .child {\
         \n  background-color: yellow;\
         \n  color: blue;\
         \n  border-color: yellow;\
         \n}\
         \n.parent {\
         \n  border-color: red;\
         \n}\n"
    );
}
