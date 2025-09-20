//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/055_test_basic_mixins.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("055_test_basic_mixins")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@mixin foo {\
             \n  .foo {a: b}}\n\
             \nbar {\
             \n  @include foo;\
             \n  c: d; }\n"),
        "bar .foo {\
         \n  a: b;\
         \n}\
         \nbar {\
         \n  c: d;\
         \n}\n"
    );
}
