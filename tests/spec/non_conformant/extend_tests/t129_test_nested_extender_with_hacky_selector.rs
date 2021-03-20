//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/129_test_nested_extender_with_hacky_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".baz .foo {a: b}\
            \nfoo + > > + bar {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".baz .foo, .baz foo + > > + bar, foo .baz + > > + bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
