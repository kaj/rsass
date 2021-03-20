//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/233_test_extend_cross_branch_redundancy_elimination.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%x .c %y {a: b}\
            \n.a, .b {@extend %x}\
            \n.a .d {@extend %y}\
            \n"
        )
        .unwrap(),
        ".a .c .d, .b .c .a .d {\
        \n  a: b;\
        \n}\
        \n"
    );
}
