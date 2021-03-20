//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/newlines_in_selectors.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo, bar\
            \nbaz {\
            \n  bang, bip\
            \n  bop {a: b}}\
            \n"
        )
        .unwrap(),
        "foo bang, foo bip\
        \nbop, bar\
        \nbaz bang, bar\
        \nbaz bip\
        \nbop {\
        \n  a: b;\
        \n}\
        \n"
    );
}
