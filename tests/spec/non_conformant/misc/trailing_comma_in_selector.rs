//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/trailing_comma_in_selector.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "#foo #bar,,\
            \n,#baz #boom, {a: b}\
            \n\
            \n#bip #bop, ,, {c: d}\
            \n"
        )
        .unwrap(),
        "#foo #bar,\
        \n#baz #boom {\
        \n  a: b;\
        \n}\
        \n#bip #bop {\
        \n  c: d;\
        \n}\
        \n"
    );
}
