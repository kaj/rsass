//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/191_test_placeholder_selector_with_multiple_extenders.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("191_test_placeholder_selector_with_multiple_extenders")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%foo {color: blue}\
             \n.bar {@extend %foo}\
             \n.baz {@extend %foo}\n"),
        ".baz, .bar {\
         \n  color: blue;\
         \n}\n"
    );
}
