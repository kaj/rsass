//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/and_and.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".and-and {\
            \n  value: true && false;\
            \n}\
            \n"
        )
        .unwrap(),
        ".and-and {\
        \n  value: true .and-and .and-and false;\
        \n}\
        \n"
    );
}
