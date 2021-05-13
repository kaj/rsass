//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/basic_mixins.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
