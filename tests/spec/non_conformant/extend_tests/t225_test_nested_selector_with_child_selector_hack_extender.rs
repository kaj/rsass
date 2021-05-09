//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/225_test_nested_selector_with_child_selector_hack_extender.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo .bar {a: b}\
             \n> foo bar {@extend .bar}\n"),
        ".foo .bar, > .foo foo bar, > foo .foo bar {\
         \n  a: b;\
         \n}\n"
    );
}
