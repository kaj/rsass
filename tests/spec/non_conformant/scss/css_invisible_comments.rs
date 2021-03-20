//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_invisible_comments.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  a: d; }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: d;\
        \n}\
        \n"
    );
}
