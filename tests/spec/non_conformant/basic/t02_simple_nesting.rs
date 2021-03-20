//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/02_simple_nesting.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  img {\
            \n    border: 0px;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div img {\
        \n  border: 0px;\
        \n}\
        \n"
    );
}
