//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/023_test_css_import_directive.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass("@import url(foo.css);").unwrap(),
        "@import url(foo.css);\
        \n"
    );
}
