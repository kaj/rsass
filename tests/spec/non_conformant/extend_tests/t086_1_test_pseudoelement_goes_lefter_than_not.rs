//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/086.1_test_pseudoelement_goes_lefter_than_not.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%a {\
            \n  x:y;\
            \n}\
            \nb:after:not(:first-child) {\
            \n  @extend %a;\
            \n}\
            \nc:s {\
            \n  @extend %a;  \
            \n}\
            \nd::e {\
            \n  @extend c;\
            \n}"
        )
        .unwrap(),
        "c:s, d:s::e, b:after:not(:first-child) {\
        \n  x: y;\
        \n}\
        \n"
    );
}
