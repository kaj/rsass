//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/placeholder.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%x {\
            \n  color: red;\
            \n}\
            \n\
            \nfoo {\
            \n  width: 10px;\
            \n  @extend %x;\
            \n}\
            \n\
            \nhux {\
            \n  height: 12px;\
            \n  @extend %x;\
            \n}"
        )
        .unwrap(),
        "hux, foo {\
        \n  color: red;\
        \n}\
        \nfoo {\
        \n  width: 10px;\
        \n}\
        \nhux {\
        \n  height: 12px;\
        \n}\
        \n"
    );
}
