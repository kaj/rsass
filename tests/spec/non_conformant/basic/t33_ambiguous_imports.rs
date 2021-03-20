//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/33_ambiguous_imports.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "main {\
            \n  color: red;\
            \n}\
            \n\
            \n@import \"dir\";"
        )
        .unwrap(),
        "main {\
        \n  color: red;\
        \n}\
        \ndir {\
        \n  color: blue;\
        \n}\
        \nfudge {\
        \n  color: brown;\
        \n}\
        \n"
    );
}
