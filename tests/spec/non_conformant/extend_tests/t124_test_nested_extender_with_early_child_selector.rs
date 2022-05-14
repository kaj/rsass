//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/124_test_nested_extender_with_early_child_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("124_test_nested_extender_with_early_child_selector")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n.bip .bar {a: b}\
             \n> .baz {@extend .bar}\
             \n}\n"),
        ".foo .bip .bar, .foo .bip .foo > .baz {\
         \n  a: b;\
         \n}\n"
    );
}
