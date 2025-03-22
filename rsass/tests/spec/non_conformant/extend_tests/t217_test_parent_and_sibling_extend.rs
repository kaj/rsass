//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/217_test_parent_and_sibling_extend.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("217_test_parent_and_sibling_extend")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            "%foo %bar%baz {c: d}\n\
             \n.parent1 {\
             \n@extend %foo;\
             \n.child1 {@extend %bar}\
             \n}\n\
             \n.parent2 {\
             \n@extend %foo;\
             \n.child2 {@extend %baz}\
             \n}\n"
        ),
        ".parent1 .parent2 .child1.child2, .parent2 .parent1 .child1.child2 {\
         \n  c: d;\
         \n}\n"
    );
}
