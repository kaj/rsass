//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/29_if.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("29_if")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n$x: a, b, 1+2;\n\
             \n@if meta.type-of(list.nth($x, 3)) == number {\
             \n  div {\
             \n    background: gray;\
             \n  }\
             \n}\n\
             \n@if meta.type-of(list.nth($x, 2)) == number {\
             \n  div {\
             \n    background: gray;\
             \n  }\
             \n}\
             \n@else if meta.type-of(list.nth($x, 2)) == string {\
             \n  div {\
             \n    background: blue;\
             \n  }\
             \n}\n\
             \n@if meta.type-of(list.nth($x, 2)) == number {\
             \n  div {\
             \n    background: gray;\
             \n  }\
             \n}\
             \n@else if meta.type-of(list.nth($x, 2)) == color {\
             \n  div {\
             \n    background: blue;\
             \n  }\
             \n}\
             \n@else {\
             \n  div {\
             \n    background: red;\
             \n  }\
             \n}"),
        "div {\
         \n  background: gray;\
         \n}\
         \ndiv {\
         \n  background: blue;\
         \n}\
         \ndiv {\
         \n  background: red;\
         \n}\n"
    );
}
