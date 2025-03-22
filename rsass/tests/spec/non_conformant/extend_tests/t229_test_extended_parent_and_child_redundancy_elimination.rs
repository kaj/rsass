//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/229_test_extended_parent_and_child_redundancy_elimination.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("229_test_extended_parent_and_child_redundancy_elimination")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a {\
             \nb {a: b}\
             \nc {@extend b}\
             \n}\
             \nd {@extend a}\n"),
        "a b, d b, a c, d c {\
         \n  a: b;\
         \n}\n"
    );
}
