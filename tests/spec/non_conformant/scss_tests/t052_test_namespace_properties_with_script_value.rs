//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/052_test_namespace_properties_with_script_value.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar: baz + bang {\
            \n    bip: bop;\
            \n    bing: bop; }}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: bazbang;\
        \n  bar-bip: bop;\
        \n  bar-bing: bop;\
        \n}\
        \n"
    );
}
