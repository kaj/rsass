//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/097_test_nested_extender.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo {a: b}\
             \nfoo bar {@extend .foo}\n"),
        ".foo, foo bar {\
         \n  a: b;\
         \n}\n"
    );
}
