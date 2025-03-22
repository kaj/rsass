//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/227_test_nested_with_child_hack_extender_and_sibling_extendee.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "227_test_nested_with_child_hack_extender_and_sibling_extendee",
    )
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("~ .foo {a: b}\
             \n> foo bar {@extend .foo}\n"),
        "~ .foo {\
         \n  a: b;\
         \n}\n"
    );
}
