//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/40_pseudo_class_identifier_starting_with_n.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div:lang(nb) {\
            \n  color: blue;\
            \n}"
        )
        .unwrap(),
        "div:lang(nb) {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
