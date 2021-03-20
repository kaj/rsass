//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/ie-backslash.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  background-color: darken(red, 10%) \\9;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  background-color: #cc0000 \\9 ;\
        \n}\
        \n"
    );
}
