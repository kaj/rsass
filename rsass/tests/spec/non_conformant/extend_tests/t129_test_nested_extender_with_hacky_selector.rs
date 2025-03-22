//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/129_test_nested_extender_with_hacky_selector.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("129_test_nested_extender_with_hacky_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".baz .foo {a: b}\
             \nfoo + > > + bar {@extend .foo}\n"),
        ".baz .foo {\
         \n  a: b;\
         \n}\n"
    );
}
