//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/mixin-content-selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo($x: 1) {\
             \n  foo-sel {\
             \n    @content;\
             \n  }\
             \n}\n\
             \ndiv {\
             \n  $x: hey;\
             \n  @include foo() {\
             \n    bar {\
             \n      color: red;\
             \n      hux {\
             \n        msg: $x;\
             \n      }\
             \n    }\
             \n  }\
             \n}"),
        "div foo-sel bar {\
         \n  color: red;\
         \n}\
         \ndiv foo-sel bar hux {\
         \n  msg: hey;\
         \n}\n"
    );
}
