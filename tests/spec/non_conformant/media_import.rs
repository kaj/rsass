//! Tests auto-converted from "sass-spec/spec/non_conformant/media_import.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass("@import \"./fonts.sass\" all;").unwrap(),
        "@import \"./fonts.sass\" all;\
        \n"
    );
}
