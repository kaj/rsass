//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/mixin-content-with-no-block.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin-content-with-no-block")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo {\
             \n  .foo {\
             \n    color: red;\
             \n    @content;\
             \n  }\
             \n}\n\
             \ndiv.a {\
             \n  @include foo() {\
             \n    hey: now;\
             \n  }\
             \n}"),
        "div.a .foo {\
         \n  color: red;\
         \n  hey: now;\
         \n}\n"
    );
}
