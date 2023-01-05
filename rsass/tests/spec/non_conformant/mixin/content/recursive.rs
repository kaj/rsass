//! Tests auto-converted from "sass-spec/spec/non_conformant/mixin/content/recursive.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("recursive")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin span($i) {\
             \n  x: y;\
             \n  @content;\
             \n}\n\
             \n.a {\
             \n  @include span(5) {\
             \n    .inner { @include span(2); }\
             \n  }\
             \n}\n"),
        ".a {\
         \n  x: y;\
         \n}\
         \n.a .inner {\
         \n  x: y;\
         \n}\n"
    );
}
