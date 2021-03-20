//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/024_test_media_import.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass("@import \"./fonts.sass\" all;").unwrap(),
        "@import \"./fonts.sass\" all;\
        \n"
    );
}
