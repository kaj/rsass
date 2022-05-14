//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/187_test_basic_placeholder_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("187_test_basic_placeholder_selector")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%foo {a: b}\
             \n.bar {@extend %foo}\n"),
        ".bar {\
         \n  a: b;\
         \n}\n"
    );
}
