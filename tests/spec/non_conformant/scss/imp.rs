//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/imp.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div { color: red; }\
            \n"
        )
        .unwrap(),
        "div {\
        \n  color: red;\
        \n}\
        \n"
    );
}
