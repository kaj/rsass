//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/091_test_redundant_selector_elimination.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("091_test_redundant_selector_elimination")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo.bar {a: b}\
             \n.x {@extend .foo, .bar}\
             \n.y {@extend .foo, .bar}\n"),
        ".foo.bar, .y, .x {\
         \n  a: b;\
         \n}\n"
    );
}
