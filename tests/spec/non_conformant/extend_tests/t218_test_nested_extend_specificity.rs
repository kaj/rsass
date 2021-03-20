//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/218_test_nested_extend_specificity.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%foo {a: b}\
            \n\
            \na {\
            \n:b {@extend %foo}\
            \n:b:c {@extend %foo}\
            \n}\
            \n"
        )
        .unwrap(),
        "a :b:c, a :b {\
        \n  a: b;\
        \n}\
        \n"
    );
}
