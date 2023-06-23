//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/218_test_nested_extend_specificity.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("218_test_nested_extend_specificity")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%foo {a: b}\n\
             \na {\
             \n:b {@extend %foo}\
             \n:b:c {@extend %foo}\
             \n}\n"),
        "a :b:c, a :b {\
         \n  a: b;\
         \n}\n"
    );
}
