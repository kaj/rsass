//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/056_test_basic_mixins.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("056_test_basic_mixins")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo {a: b}\n\
             \nbar {\
             \n  @include foo;\
             \n  c: d; }\n"),
        "bar {\
         \n  a: b;\
         \n  c: d;\
         \n}\n"
    );
}
