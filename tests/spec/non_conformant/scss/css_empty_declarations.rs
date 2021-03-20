//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_empty_declarations.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  bar: baz; }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}
