//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/224_test_nested_selector_with_child_selector_hack_extendee.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "224_test_nested_selector_with_child_selector_hack_extendee",
    )
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("> .foo {a: b}\
             \nfoo bar {@extend .foo}\n"),
        "> .foo, > foo bar {\
         \n  a: b;\
         \n}\n"
    );
}
