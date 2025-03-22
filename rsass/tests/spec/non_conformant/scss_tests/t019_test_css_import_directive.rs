//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/019_test_css_import_directive.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("019_test_css_import_directive")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \"foo.css\";"),
        "@import \"foo.css\";\n"
    );
}
