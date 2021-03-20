//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/backslash.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div, span {\
            \n\tcolor: red;\
            \n\t\\ foo {\
            \n\t\tcolor: blue;\
            \n\t}\
            \n}"
        )
        .unwrap(),
        "div, span {\
        \n  color: red;\
        \n}\
        \ndiv \\ foo, span \\ foo {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
