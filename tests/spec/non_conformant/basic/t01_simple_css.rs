//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/01_simple_css.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  color: blue;\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
