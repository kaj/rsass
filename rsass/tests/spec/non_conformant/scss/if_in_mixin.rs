//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/if-in-mixin.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("if-in-mixin")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$x: true;\n\
             \n@mixin foobar() {\
             \n  @if $x {\
             \n    $x: false !global;\
             \n    content: foo;\
             \n  }\
             \n  @else {\
             \n    $x: true !global;\
             \n    content: bar;\
             \n  }\
             \n}\n\
             \ndiv {\
             \n  @include foobar();\
             \n  @include foobar();\
             \n  @include foobar();\
             \n  $x: true !global;\
             \n  @include foobar();\
             \n}\n"),
        "div {\
         \n  content: foo;\
         \n  content: bar;\
         \n  content: foo;\
         \n  content: foo;\
         \n}\n"
    );
}
