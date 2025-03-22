//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/126_test_nested_extender_with_early_child_selector.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("126_test_nested_extender_with_early_child_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo + .bar {a: b}\
             \n.bip > .baz {@extend .bar}\n"),
        ".foo + .bar, .bip > .foo + .baz {\
         \n  a: b;\
         \n}\n"
    );
}
