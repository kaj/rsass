//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/extend-self.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("// This shouldn\'t change the selector.\
             \n.c, .a .b .c, .a .c .b {x: y; @extend .c}\n"),
        ".c, .a .b .c, .a .c .b {\
         \n  x: y;\
         \n}\n"
    );
}
