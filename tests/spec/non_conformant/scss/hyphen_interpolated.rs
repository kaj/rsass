//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/hyphen-interpolated.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  foo: -hux-#{2+3};\
            \n  bar: hux-#{2+3};\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo: -hux-5;\
        \n  bar: hux-5;\
        \n}\
        \n"
    );
}
