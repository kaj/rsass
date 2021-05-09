//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_selector_hacks.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("> > E {\
             \n  a: b; }\n"),
        "> > E {\
         \n  a: b;\
         \n}\n"
    );
}
