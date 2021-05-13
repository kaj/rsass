//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/020_test_css_import_directive.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \'foo.css\';"),
        "@import \'foo.css\';\n"
    );
}
