//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_spaceless_combo_selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
