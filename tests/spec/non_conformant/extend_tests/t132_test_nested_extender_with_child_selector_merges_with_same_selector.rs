//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/132_test_nested_extender_with_child_selector_merges_with_same_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("132_test_nested_extender_with_child_selector_merges_with_same_selector")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo > .bar .baz {a: b}\
             \n.foo > .bar .bang {@extend .baz}\n"),
        ".foo > .bar .baz, .foo > .bar .bang {\
         \n  a: b;\
         \n}\n"
    );
}
