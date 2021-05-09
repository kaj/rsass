//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/024_test_media_import.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \"./fonts.sass\" all;"),
        "@import \"./fonts.sass\" all;\n"
    );
}
