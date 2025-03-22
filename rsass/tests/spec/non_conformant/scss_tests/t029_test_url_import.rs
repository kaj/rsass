//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/029_test_url_import.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("029_test_url_import")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import url(fonts.sass);"),
        "@import url(fonts.sass);\n"
    );
}
