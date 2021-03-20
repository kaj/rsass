//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/028_test_import_with_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$family: unquote(\"Droid+Sans\");\
            \n@import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\
            \n"
        )
        .unwrap(),
        "@import url(\"http://fonts.googleapis.com/css?family=Droid+Sans\");\
        \n"
    );
}
