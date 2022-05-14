//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/url_import.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("url_import")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import url(fonts.sass);"),
        "@import url(fonts.sass);\n"
    );
}
