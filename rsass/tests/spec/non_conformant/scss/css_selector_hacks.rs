//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_selector_hacks.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_selector_hacks")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("> > E {\
             \n  a: b; }\n"),
        ""
    );
}
