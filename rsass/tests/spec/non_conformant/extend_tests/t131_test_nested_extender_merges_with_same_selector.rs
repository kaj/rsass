//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/131_test_nested_extender_merges_with_same_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("131_test_nested_extender_merges_with_same_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n.bar {a: b}\
             \n.baz {@extend .bar} }\n"),
        ".foo .bar, .foo .baz {\
         \n  a: b;\
         \n}\n"
    );
}
