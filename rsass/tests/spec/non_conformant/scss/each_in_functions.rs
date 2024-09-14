//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/each_in_functions.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("each_in_functions")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n$GLOBAL: global;\n\
             \n@function foo($g1, $g2, $g3) {\
             \n  @each $value in $g1, $g2, $g3 {\
             \n    $GLOBAL: $GLOBAL each $value !global;\
             \n    $GLOBAL: $GLOBAL type1 meta.type-of(list.nth($value, 1)) !global;\
             \n    $GLOBAL: $GLOBAL type2 meta.type-of(list.nth($value, 2)) !global;\
             \n  }\
             \n  @each $value in (foo: foo, bar: bar) {\
             \n    $GLOBAL: $GLOBAL map $value !global;\
             \n  }\
             \n  @return 0;\
             \n}\n\
             \ndiv {\
             \n  a: foo(50% 50%, cover circle, red blue);\
             \n  b: $GLOBAL;\
             \n  $colors: red green blue;\
             \n  c: a, b, meta.type-of(list.nth($colors, 2)), d;\
             \n}\n"
        ),
        "div {\
         \n  a: 0;\
         \n  b: global each 50% 50% type1 number type2 number each cover circle type1 string type2 string each red blue type1 color type2 color map foo foo map bar bar;\
         \n  c: a, b, color, d;\
         \n}\n"
    );
}
