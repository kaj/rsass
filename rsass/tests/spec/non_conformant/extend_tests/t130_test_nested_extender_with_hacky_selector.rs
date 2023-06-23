//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/130_test_nested_extender_with_hacky_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("130_test_nested_extender_with_hacky_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".baz .foo {a: b}\
             \n> > bar {@extend .foo}\n"),
        ".baz .foo {\
         \n  a: b;\
         \n}\n"
    );
}
