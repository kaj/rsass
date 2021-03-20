//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_basic_scss.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "sel {\
            \n  p: v; }\
            \n"
        )
        .unwrap(),
        "sel {\
        \n  p: v;\
        \n}\
        \n"
    );
}
