//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/18_mixin_scope.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("18_mixin_scope")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$x: global x;\
             \n$y: global y;\n\
             \n@mixin foo($x) {\
             \n  f-a: $x;\
             \n  f-b: $y;\
             \n  $x: local x changed by foo;\
             \n  $y: global y changed by foo !global;\
             \n  $z: new local z;\
             \n  f-a: $x;\
             \n  f-b: $y;\
             \n  f-c: $z;\
             \n}\n\
             \ndiv {\
             \n  a: $x;\
             \n  b: $y;\
             \n  @include foo(arg);\
             \n  a: $x;\
             \n  b: $y;\
             \n}\n"),
        "div {\
         \n  a: global x;\
         \n  b: global y;\
         \n  f-a: arg;\
         \n  f-b: global y;\
         \n  f-a: local x changed by foo;\
         \n  f-b: global y changed by foo;\
         \n  f-c: new local z;\
         \n  a: global x;\
         \n  b: global y changed by foo;\
         \n}\n"
    );
}
