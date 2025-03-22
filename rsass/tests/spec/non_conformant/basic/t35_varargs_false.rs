//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/35_varargs_false.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("35_varargs_false")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo($args...) {\
             \n  @each $arg in $args {\
             \n    @if $arg {\
             \n      thing: $arg;\
             \n    }\
             \n  }\
             \n}\n\
             \ndiv {\
             \n  @include foo(a, b, false);\
             \n}\n"),
        "div {\
         \n  thing: a;\
         \n  thing: b;\
         \n}\n"
    );
}
