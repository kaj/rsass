//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/several_namespace_properties.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar: baz;\
            \n  bang: {\
            \n    bip: 1px;\
            \n    bop: bar;}\
            \n  buzz: {\
            \n    fram: \"foo\";\
            \n    frum: moo;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n  bang-bip: 1px;\
        \n  bang-bop: bar;\
        \n  buzz-fram: \"foo\";\
        \n  buzz-frum: moo;\
        \n}\
        \n"
    );
}
