//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_spaceless_combo_selectors.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "E + F {\
            \n  a: b; }\
            \n"
        )
        .unwrap(),
        "E + F {\
        \n  a: b;\
        \n}\
        \n"
    );
}
