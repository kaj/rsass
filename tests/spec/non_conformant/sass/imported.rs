//! Tests auto-converted from "sass-spec/spec/non_conformant/sass/imported.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"imported.sass\";\
            \n"
        )
        .unwrap(),
        "div a {\
        \n  color: red;\
        \n}\
        \ndiv li {\
        \n  color: green;\
        \n}\
        \n"
    );
}
