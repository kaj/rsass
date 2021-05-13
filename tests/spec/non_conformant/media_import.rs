//! Tests auto-converted from "sass-spec/spec/non_conformant/media_import.hrx"

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
