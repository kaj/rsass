//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/190_test_semi_placeholder_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("190_test_semi_placeholder_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("#context %foo, .bar .baz {color: blue}\n\
             \n.bat {\
             \n  @extend %foo;\
             \n}\n"),
        "#context .bat, .bar .baz {\
         \n  color: blue;\
         \n}\n"
    );
}
