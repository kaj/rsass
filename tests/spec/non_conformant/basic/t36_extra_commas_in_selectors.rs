//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/36_extra_commas_in_selectors.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div,, , span, ,, {\
            \n  color: red;\
            \n}"
        )
        .unwrap(),
        "div, span {\
        \n  color: red;\
        \n}\
        \n"
    );
}
