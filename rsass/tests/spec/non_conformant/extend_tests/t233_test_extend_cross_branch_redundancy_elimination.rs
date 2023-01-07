//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/233_test_extend_cross_branch_redundancy_elimination.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("233_test_extend_cross_branch_redundancy_elimination")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%x .c %y {a: b}\
             \n.a, .b {@extend %x}\
             \n.a .d {@extend %y}\n"),
        ".a .c .d, .b .c .a .d {\
         \n  a: b;\
         \n}\n"
    );
}
