//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/120_test_nested_extender_with_child_selector_unifies.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("120_test_nested_extender_with_child_selector_unifies")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".baz.foo {a: b}\
             \nfoo > bar {@extend .foo}\n"),
        ".baz.foo, foo > bar.baz {\
         \n  a: b;\
         \n}\n"
    );
}
