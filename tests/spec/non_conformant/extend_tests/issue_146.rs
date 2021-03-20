//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/issue_146.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%btn-style-default {\
            \n  background: green;\
            \n  &:hover{\
            \n    background: black;\
            \n  }\
            \n}\
            \n\
            \nbutton {\
            \n  @extend %btn-style-default;\
            \n}"
        )
        .unwrap(),
        "button {\
        \n  background: green;\
        \n}\
        \nbutton:hover {\
        \n  background: black;\
        \n}\
        \n"
    );
}
