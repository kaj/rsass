//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/225_test_nested_selector_with_child_selector_hack_extender.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo .bar {a: b}\
            \n> foo bar {@extend .bar}\
            \n"
        )
        .unwrap(),
        ".foo .bar, > .foo foo bar, > foo .foo bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
