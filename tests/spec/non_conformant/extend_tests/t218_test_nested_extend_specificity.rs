//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/218_test_nested_extend_specificity.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
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
