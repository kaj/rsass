//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/029_test_url_import.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass("@import url(fonts.sass);").unwrap(),
        "@import url(fonts.sass);\
        \n"
    );
}
