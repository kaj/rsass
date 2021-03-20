//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/03_simple_variable.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$color: red;\
            \n\
            \na {\
            \n  color: $color;\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  color: red;\
        \n}\
        \n"
    );
}
