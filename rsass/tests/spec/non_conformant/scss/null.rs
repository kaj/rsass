//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/null.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("null")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n$x: 2;\n\
             \ndiv {\
             \n  $x: null;\
             \n  a: list.length(null null null);\
             \n  b: #{null};\
             \n  d: meta.type-of($x);\
             \n  e: null == null;\
             \n  f: -null;\
             \n  g: -fudge;\
             \n  h: (null null null);\
             \n  i: froo(null, 4);\
             \n  j: (null), (null), 3, 4;\
             \n  k: list.length(((null), (null), 3, 4));\n\
             \n  a2: list.length($x $x $x);\
             \n  b2: #{$x};\
             \n  e2: $x == null;\
             \n  f2: -$x;\
             \n  h2: ($x $x $x);\
             \n  i2: froo($x, 4);\
             \n  j2: ($x), ($x), 3, 4;\
             \n  k2: list.length((($x), ($x), 3, 4));\
             \n}"),
        "div {\
         \n  a: 3;\
         \n  d: null;\
         \n  e: true;\
         \n  f: -null;\
         \n  g: -fudge;\
         \n  i: froo(, 4);\
         \n  j: 3, 4;\
         \n  k: 4;\
         \n  a2: 3;\
         \n  e2: true;\
         \n  f2: -;\
         \n  i2: froo(, 4);\
         \n  j2: 3, 4;\
         \n  k2: 4;\
         \n}\n"
    );
}
