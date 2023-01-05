//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/arglist.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("arglist")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo($x, $y, $zs...) {\
             \n  foo-x: $x;\
             \n  foo-y: $y;\
             \n  foo-zs: $zs;\
             \n}\n\
             \ndiv {\
             \n  @include foo(a, b, c, d, e);\
             \n}"),
        "div {\
         \n  foo-x: a;\
         \n  foo-y: b;\
         \n  foo-zs: c, d, e;\
         \n}\n"
    );
}
