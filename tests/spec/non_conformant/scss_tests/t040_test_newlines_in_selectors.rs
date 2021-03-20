//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/040_test_newlines_in_selectors.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo\
            \nbar {a: b}\
            \n"
        )
        .unwrap(),
        "foo\
        \nbar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
