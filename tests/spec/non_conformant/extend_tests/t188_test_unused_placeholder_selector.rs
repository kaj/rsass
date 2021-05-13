//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/188_test_unused_placeholder_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%foo {color: blue}\
             \n%bar {color: red}\
             \n.baz {@extend %foo}\n"),
        ".baz {\
         \n  color: blue;\
         \n}\n"
    );
}
