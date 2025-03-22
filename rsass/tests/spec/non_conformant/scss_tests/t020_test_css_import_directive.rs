//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/020_test_css_import_directive.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("020_test_css_import_directive")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \'foo.css\';"),
        "@import \'foo.css\';\n"
    );
}
