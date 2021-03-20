//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/extend-self.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "// This shouldn\'t change the selector.\
            \n.c, .a .b .c, .a .c .b {x: y; @extend .c}\
            \n"
        )
        .unwrap(),
        ".c, .a .b .c, .a .c .b {\
        \n  x: y;\
        \n}\
        \n"
    );
}
