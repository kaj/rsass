//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/189_test_placeholder_descendant_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "#context %foo a {a: b}\
            \n.bar {@extend %foo}\
            \n"
        )
        .unwrap(),
        "#context .bar a {\
        \n  a: b;\
        \n}\
        \n"
    );
}
