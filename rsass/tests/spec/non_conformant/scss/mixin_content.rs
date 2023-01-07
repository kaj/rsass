//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/mixin-content.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin-content")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin foo() {\
             \n  name: foo;\
             \n  @content;\
             \n  @include bar() {\
             \n    stuff: content for bar;\
             \n    @content;\
             \n  }\
             \n}\n\
             \n@mixin bar() {\
             \n  name: bar;\
             \n  @content;\
             \n}\n\
             \ndiv {\
             \n  /* with a content block */\
             \n  @include foo() {\
             \n    stuff: content for foo;\
             \n  }\
             \n  /* without */\
             \n  @include foo();\
             \n}\n\
             \n@mixin foo() {\
             \n  $size: 80%;\
             \n  div {\
             \n    color: red;\
             \n    @content;\
             \n    background: blue;\
             \n    width: $size;\
             \n    @include bar() {\
             \n      color: orange;\
             \n      @content;\
             \n    }\
             \n  }\
             \n}\n\
             \n@mixin bar() {\
             \n  form {\
             \n    @content;\
             \n  }\
             \n}\n\
             \nspan {\
             \n  $size: 1.2em;\
             \n  color: green;\
             \n  @include foo() {\
             \n    @media fudge {\
             \n      p {\
             \n        font-weight: bold;\
             \n        font-size: $size;\
             \n        a {\
             \n          text-decoration: underline;\
             \n        }\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@mixin apply-to-ie6-only {\
             \n  * html {\
             \n    @content;\
             \n  }\
             \n}\
             \n@include apply-to-ie6-only {\
             \n  #logo {\
             \n    background-image: url(/logo.gif);\
             \n  }\
             \n}\n"),
        "div {\
         \n  /* with a content block */\
         \n  name: foo;\
         \n  stuff: content for foo;\
         \n  name: bar;\
         \n  stuff: content for bar;\
         \n  stuff: content for foo;\
         \n  /* without */\
         \n  name: foo;\
         \n  name: bar;\
         \n  stuff: content for bar;\
         \n}\
         \nspan {\
         \n  color: green;\
         \n}\
         \nspan div {\
         \n  color: red;\
         \n  background: blue;\
         \n  width: 80%;\
         \n}\
         \n@media fudge {\
         \n  span div p {\
         \n    font-weight: bold;\
         \n    font-size: 1.2em;\
         \n  }\
         \n  span div p a {\
         \n    text-decoration: underline;\
         \n  }\
         \n}\
         \nspan div form {\
         \n  color: orange;\
         \n}\
         \n@media fudge {\
         \n  span div form p {\
         \n    font-weight: bold;\
         \n    font-size: 1.2em;\
         \n  }\
         \n  span div form p a {\
         \n    text-decoration: underline;\
         \n  }\
         \n}\
         \n* html #logo {\
         \n  background-image: url(/logo.gif);\
         \n}\n"
    );
}
