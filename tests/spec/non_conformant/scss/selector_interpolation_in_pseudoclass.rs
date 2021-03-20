//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/selector_interpolation_in_pseudoclass.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo:nth-child(#{5 + \"n\"}) {a: b}\
            \n"
        )
        .unwrap(),
        "foo:nth-child(5n) {\
        \n  a: b;\
        \n}\
        \n"
    );
}
