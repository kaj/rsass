//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_empty_rule.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_empty_rule")
}

#[test]
fn test() {
    assert_eq!(runner().ok(""), "");
}
