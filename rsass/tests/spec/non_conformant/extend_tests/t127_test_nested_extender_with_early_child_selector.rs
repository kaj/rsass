//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/127_test_nested_extender_with_early_child_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("127_test_nested_extender_with_early_child_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo > .bar {a: b}\
             \n.bip > .baz {@extend .bar}\n"),
        ".foo > .bar, .foo.bip > .baz {\
         \n  a: b;\
         \n}\n"
    );
}
