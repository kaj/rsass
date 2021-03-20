//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/053_test_no_namespace_properties_without_space.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar:baz {\
            \n    bip: bop }}\
            \n"
        )
        .unwrap(),
        "foo bar:baz {\
        \n  bip: bop;\
        \n}\
        \n"
    );
}
