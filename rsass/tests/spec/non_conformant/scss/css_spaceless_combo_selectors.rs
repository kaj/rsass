//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_spaceless_combo_selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_spaceless_combo_selectors")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("E + F {\
             \n  a: b; }\n"),
        "E + F {\
         \n  a: b;\
         \n}\n"
    );
}
