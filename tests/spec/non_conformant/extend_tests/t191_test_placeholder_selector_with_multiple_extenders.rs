//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/191_test_placeholder_selector_with_multiple_extenders.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
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
