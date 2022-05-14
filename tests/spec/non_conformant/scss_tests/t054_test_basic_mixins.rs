//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/054_test_basic_mixins.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("054_test_basic_mixins")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo {\
             \n  .foo {a: b}}\n\
             \n@include foo;\n"),
        ".foo {\
         \n  a: b;\
         \n}\n"
    );
}
