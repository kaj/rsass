//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/234_test_extend_cross_branch_redundancy_elimination.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".e %z {a: b}\
            \n%x .c %y {@extend %z}\
            \n.a, .b {@extend %x}\
            \n.a .d {@extend %y}\
            \n"
        )
        .unwrap(),
        ".e .a .c .d, .e .b .c .a .d, .a .e .b .c .d, .a .c .e .d, .b .c .e .a .d {\
        \n  a: b;\
        \n}\
        \n"
    );
}
