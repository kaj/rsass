//! Tests auto-converted from "sass-spec/spec/libsass/delayed.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("delayed")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n$x: a 3/4 b;\
             \n$y: hey;\n\
             \n@function foo() {\
             \n  @return 3/4;\
             \n}\n\
             \ndiv {\
             \n  hoo: 3/4;\
             \n  goo: list.nth($x, 2);\
             \n  foo: 15 / list.nth($x, 2);\
             \n  foo: .25 + list.nth($x, 2);\
             \n  coo: 2/3 / list.nth($x, 2);\
             \n  bar: $y and true;\
             \n  bar: false and true;\
             \n  bar: (false) and true;\
             \n  @each $elem in $x {\
             \n    blah: $elem;\
             \n  }\
             \n  bloo: foo();\
             \n  @warn 2/3;\
             \n  blix: \"hey #{list.nth($x, 2)} ho\";\
             \n}\n\
             \n@media screen and (hux: 3/4) {\
             \n  div {\
             \n    color: red;\
             \n  }\
             \n}\n\
             \n@warn \"blah blah\";\n\
             \ndiv {\
             \n  blah: \"ho #{list.nth($x, 2) } ho\";\
             \n}\n\
             \nspan {\
             \n  fludge: (true and 3/4);\
             \n}"),
        "div {\
         \n  hoo: 3/4;\
         \n  goo: 0.75;\
         \n  foo: 20;\
         \n  foo: 1;\
         \n  coo: 0.8888888889;\
         \n  bar: true;\
         \n  bar: false;\
         \n  bar: false;\
         \n  blah: a;\
         \n  blah: 0.75;\
         \n  blah: b;\
         \n  bloo: 0.75;\
         \n  blix: \"hey 0.75 ho\";\
         \n}\
         \n@media screen and (hux: 3/4) {\
         \n  div {\
         \n    color: red;\
         \n  }\
         \n}\
         \ndiv {\
         \n  blah: \"ho 0.75 ho\";\
         \n}\
         \nspan {\
         \n  fludge: 0.75;\
         \n}\n"
    );
}
