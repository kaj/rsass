//! Tests auto-converted from "sass-spec/spec/libsass/unary-ops.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unary-ops")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n$x: 20%;\n\
             \ndiv {\
             \n  a: -10;\
             \n  b: -10px + 10px;\
             \n  c: +10;\
             \n  d: +10px + -10px;\
             \n  e: -$x;\
             \n  f: +$x;\
             \n  g: -hello;\
             \n  h: +hello;\
             \n  i: + hello;\
             \n  j: meta.type-of(+ hello);\
             \n}"),
        "div {\
         \n  a: -10;\
         \n  b: 0px;\
         \n  c: 10;\
         \n  d: 0px;\
         \n  e: -20%;\
         \n  f: 20%;\
         \n  g: -hello;\
         \n  h: +hello;\
         \n  i: +hello;\
         \n  j: string;\
         \n}\n"
    );
}
