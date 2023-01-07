//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/234_test_extend_cross_branch_redundancy_elimination.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("234_test_extend_cross_branch_redundancy_elimination")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            ".e %z {a: b}\
             \n%x .c %y {@extend %z}\
             \n.a, .b {@extend %x}\
             \n.a .d {@extend %y}\n"
        ),
        ".e .a .c .d, .e .b .c .a .d, .a .e .b .c .d, .a .c .e .d, .b .c .e .a .d {\
         \n  a: b;\
         \n}\n"
    );
}
