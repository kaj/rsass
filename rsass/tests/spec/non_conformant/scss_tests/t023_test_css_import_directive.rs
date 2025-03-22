//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/023_test_css_import_directive.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("023_test_css_import_directive")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import url(foo.css);"),
        "@import url(foo.css);\n"
    );
}
