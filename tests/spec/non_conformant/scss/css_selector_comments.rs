//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_selector_comments.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo #bar:baz(bip) {\
             \n  a: b; }\n"),
        ".foo #bar:baz(bip) {\
         \n  a: b;\
         \n}\n"
    );
}
