//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/228_test_nested_with_child_selector_hack_extender_extendee_newline.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "> .foo {a: b}\
            \nflip,\
            \n> foo bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        "> .foo, > flip,\
        \n> foo bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
