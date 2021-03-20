//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/function-names.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  color: unquote(\"hello\");\
            \n  color: un#{quo}te(\"hello\");\
            \n  color: (\"hello\")un#{quo}te;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  color: hello;\
        \n  color: unquote(\"hello\");\
        \n  color: \"hello\" unquote;\
        \n}\
        \n"
    );
}
