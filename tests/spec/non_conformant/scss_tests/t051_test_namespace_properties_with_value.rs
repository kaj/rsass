//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/051_test_namespace_properties_with_value.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar: baz {\
            \n    bip: bop;\
            \n    bing: bop; }}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n  bar-bip: bop;\
        \n  bar-bing: bop;\
        \n}\
        \n"
    );
}
