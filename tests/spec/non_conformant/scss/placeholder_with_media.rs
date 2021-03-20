//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/placeholder-with-media.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%a {\
            \n  @media only screen and (max-width: 100px) {\
            \n    color: red;\
            \n  }\
            \n}\
            \n\
            \nb {\
            \n  @extend %a;\
            \n}\
            \n"
        )
        .unwrap(),
        "@media only screen and (max-width: 100px) {\
        \n  b {\
        \n    color: red;\
        \n  }\
        \n}\
        \n"
    );
}
