//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/027_test_protocol_relative_import.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"//fonts.googleapis.com/css?family=Droid+Sans\";"
        )
        .unwrap(),
        "@import \"//fonts.googleapis.com/css?family=Droid+Sans\";\
        \n"
    );
}
