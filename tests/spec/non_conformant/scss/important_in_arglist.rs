//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/important-in-arglist.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo($x) {\
             \n  style: $x;\
             \n}\n\
             \ndiv {\
             \n  @include foo(0px 0px 0px 0px #ef8086 inset !important);\
             \n  fludge: foo bar ! important hux;\
             \n}"),
        "div {\
         \n  style: 0px 0px 0px 0px #ef8086 inset !important;\
         \n  fludge: foo bar !important hux;\
         \n}\n"
    );
}
