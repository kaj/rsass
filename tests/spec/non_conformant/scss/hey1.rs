//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/hey1.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div { width: 1px; }\
            \n"
        )
        .unwrap(),
        "div {\
        \n  width: 1px;\
        \n}\
        \n"
    );
}
