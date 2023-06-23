//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/122_test_nested_extender_with_child_selector_unifies.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("122_test_nested_extender_with_child_selector_unifies")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n.bar {a: b}\
             \n> .baz {@extend .bar}\
             \n}\n"),
        ".foo .bar, .foo > .baz {\
         \n  a: b;\
         \n}\n"
    );
}
