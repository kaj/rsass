//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/023_test_css_import_directive.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import url(foo.css);"),
        "@import url(foo.css);\n"
    );
}
