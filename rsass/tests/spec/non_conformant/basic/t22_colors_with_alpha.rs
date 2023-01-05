//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/22_colors_with_alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("22_colors_with_alpha")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$x: rgb(0, 255, 255);\n\
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
             \n  hoo: red($x);\
             \n  moo: green($x);\
             \n  poo: blue($x);\
             \n  \
             \n  goo: mix(rgba(255, 0, 0, 0.5), #00f);\
             \n  \
             \n  boo: invert(#123456);\
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
         \n  goo: rgba(64, 0, 191, 0.75);\
         \n  boo: #edcba9;\
         \n}\n"
    );
}
