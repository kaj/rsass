//! Tests auto-converted from "sass-spec/spec/non_conformant/media_import.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("media_import")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \"./fonts.sass\" all;"),
        "@import \"./fonts.sass\" all;\n"
    );
}
