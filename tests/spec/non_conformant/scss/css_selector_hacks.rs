//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_selector_hacks.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "> > E {\
            \n  a: b; }\
            \n"
        )
        .unwrap(),
        "> > E {\
        \n  a: b;\
        \n}\
        \n"
    );
}
