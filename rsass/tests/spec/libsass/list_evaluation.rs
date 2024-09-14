//! Tests auto-converted from "sass-spec/spec/libsass/list-evaluation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("list-evaluation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \ndiv {\
             \n  $things: red 2/3 blue;\
             \n  content: $things;\
             \n  content: list.nth($things, 2);\
             \n  content: meta.type-of(list.nth($things, 2));\
             \n  content: meta.type-of(list.nth($things, 3));\
             \n  /**** #{2+2} ****/\
             \n  content: (1 / 2 3 / 4) + (5/6 7/8);\
             \n  content: (1/2 3/4), (5/6 7/8);\
             \n  /**** ****/\
             \n  @each $x in 1 2 3/4 {\
             \n    foo: $x;\
             \n    bar: $x + 1;\
             \n  }\
             \n  /*** ***/\
             \n  stuff: 1, (2 3/4 5), 6;\
             \n  stuff: ((1 + 2)/3/4);\
             \n}"),
        "div {\
         \n  content: red 2/3 blue;\
         \n  content: 0.6666666667;\
         \n  content: number;\
         \n  content: color;\
         \n  /**** 4 ****/\
         \n  content: 1/2 3/45/6 7/8;\
         \n  content: 1/2 3/4, 5/6 7/8;\
         \n  /**** ****/\
         \n  foo: 1;\
         \n  bar: 2;\
         \n  foo: 2;\
         \n  bar: 3;\
         \n  foo: 0.75;\
         \n  bar: 1.75;\
         \n  /*** ***/\
         \n  stuff: 1, 2 3/4 5, 6;\
         \n  stuff: 0.25;\
         \n}\n"
    );
}
