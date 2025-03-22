//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/226_test_nested_selector_with_child_selector_hack_extender_and_extendee.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("226_test_nested_selector_with_child_selector_hack_extender_and_extendee")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("> .foo {a: b}\
             \n> foo bar {@extend .foo}\n"),
        "> .foo, > foo bar {\
         \n  a: b;\
         \n}\n"
    );
}
