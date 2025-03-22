//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_selector_comments.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_selector_comments")
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
