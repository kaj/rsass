//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_selector_comments.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo #bar:baz(bip) {\
            \n  a: b; }\
            \n"
        )
        .unwrap(),
        ".foo #bar:baz(bip) {\
        \n  a: b;\
        \n}\
        \n"
    );
}
