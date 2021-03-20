//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/020_test_css_import_directive.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass("@import \'foo.css\';").unwrap(),
        "@import \'foo.css\';\
        \n"
    );
}
