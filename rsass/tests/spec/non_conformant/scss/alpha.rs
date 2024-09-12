//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \n$x: rgb(0, 255, 255);\n\
             \ndiv {\
             \n  color: rgb(255, $blue: 0, $green: 255);\
             \n  background: rgb(123, 45, 6);\
             \n//  flah: rgba(0, 0, 0, 1) + #111;\
             \n  grah: rgba(#f0e, $alpha: .5);\
             \n//  blah: rgba(1,2,3,.6);\
             \n  \
             \n  floo: $x;\
             \n//  bloo: rgba($x, 0.7);\
             \n  groo: $x;\
             \n  \
             \n  $x: rgb(123, 45, 6);\
             \n  \
             \n  hoo: color.red($x);\
             \n  moo: color.green($x);\
             \n  poo: color.blue($x);\
             \n  \
             \n//  goo: mix(rgba(255, 0, 0, 0.5), #00f);\
             \n  \
             \n  boo: color.invert(#123456);\
             \n}\n"),
        "div {\
         \n  color: rgb(255, 255, 0);\
         \n  background: rgb(123, 45, 6);\
         \n  grah: rgba(255, 0, 238, 0.5);\
         \n  floo: rgb(0, 255, 255);\
         \n  groo: rgb(0, 255, 255);\
         \n  hoo: 123;\
         \n  moo: 45;\
         \n  poo: 6;\
         \n  boo: #edcba9;\
         \n}\n"
    );
}
