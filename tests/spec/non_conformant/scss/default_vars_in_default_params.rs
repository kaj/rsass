//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/default-vars-in-default-params.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("default-vars-in-default-params")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$y: why;\n\
             \n@mixin foo($x, $y: $y) {\
             \n  stuff: $x $y;\
             \n}\n\
             \ndiv {\
             \n  why: $y;\
             \n  @include foo(ecks);\
             \n}"),
        "div {\
         \n  why: why;\
         \n  stuff: ecks why;\
         \n}\n"
    );
}
