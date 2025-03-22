//! Tests auto-converted from "sass-spec/spec/non_conformant/mixin/content/before_if.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("before_if")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("// Regression test for sass/dart-sass#482.\
             \n@mixin outer {\
             \n  a {@content}\
             \n}\n\
             \n@mixin inner {\
             \n  @content;\
             \n}\n\
             \n@include outer {\
             \n  @include inner {}\
             \n  x: y;\
             \n}"),
        "a {\
         \n  x: y;\
         \n}\n"
    );
}
