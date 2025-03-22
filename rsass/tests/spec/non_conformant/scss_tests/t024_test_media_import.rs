//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/024_test_media_import.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("024_test_media_import")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \"./fonts.sass\" all;"),
        "@import \"./fonts.sass\" all;\n"
    );
}
