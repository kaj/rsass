//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/simple-lists.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  hey: a, b, c, d;\
            \n  ho: a b c d;\
            \n  ha: unquote(\"a, b, c, d\");\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  hey: a, b, c, d;\
        \n  ho: a b c d;\
        \n  ha: a, b, c, d;\
        \n}\
        \n"
    );
}
