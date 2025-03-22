//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/121_test_nested_extender_with_child_selector_unifies.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("121_test_nested_extender_with_child_selector_unifies")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".baz > {\
             \n.foo {a: b}\
             \n.bar {@extend .foo}\
             \n}\n"),
        ".baz > .foo, .baz > .bar {\
         \n  a: b;\
         \n}\n"
    );
}
