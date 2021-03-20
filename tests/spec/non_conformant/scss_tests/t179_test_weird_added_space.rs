//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/179_test_weird_added_space.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$value : bip;\
            \n\
            \nfoo {\
            \n  bar: -moz-#{$value};\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: -moz-bip;\
        \n}\
        \n"
    );
}
