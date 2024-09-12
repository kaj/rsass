//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/22_colors_with_alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("22_colors_with_alpha")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \n$x: rgb(0, 255, 255);\n\
             \ndiv {\
             \n  color: rgb(255, $blue: 0, $green: 255);\
             \n  background: rgb(123, 45, 6);\
             \n  grah: rgba(#f0e, $alpha: .5);\
             \n  blah: rgba(1,2,3,.6);\
             \n  \
             \n  floo: $x;\
             \n  bloo: rgba($x, 0.7);\
             \n  groo: $x;\
             \n  \
             \n  $x: rgb(123, 45, 6);\
             \n  \
             \n  hoo: color.red($x);\
             \n  moo: color.green($x);\
             \n  poo: color.blue($x);\
             \n  \
             \n  goo: color.mix(rgba(255, 0, 0, 0.5), #00f);\
             \n  \
             \n  boo: color.invert(#123456);\
             \n}\n"),
        "div {\
         \n  color: rgb(255, 255, 0);\
         \n  background: rgb(123, 45, 6);\
         \n  grah: rgba(255, 0, 238, 0.5);\
         \n  blah: rgba(1, 2, 3, 0.6);\
         \n  floo: rgb(0, 255, 255);\
         \n  bloo: rgba(0, 255, 255, 0.7);\
         \n  groo: rgb(0, 255, 255);\
         \n  hoo: 123;\
         \n  moo: 45;\
         \n  poo: 6;\
         \n  goo: rgba(63.75, 0, 191.25, 0.75);\
         \n  boo: #edcba9;\
         \n}\n"
    );
}
