//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/228_test_nested_with_child_selector_hack_extender_extendee_newline.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd(
        "228_test_nested_with_child_selector_hack_extender_extendee_newline",
    )
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("> .foo {a: b}\
             \nflip,\
             \n> foo bar {@extend .foo}\n"),
        "> .foo, > flip,\
         \n> foo bar {\
         \n  a: b;\
         \n}\n"
    );
}
