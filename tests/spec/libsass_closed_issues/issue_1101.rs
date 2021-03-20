//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1101.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: white;\r\
            \nfoo {\r\
            \n  bar: adjust-color($foo, $hue: -6deg, $lightness: -16%, $saturation: -7%);\r\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  bar: #d6d6d6;\
        \n}\
        \n"
    );
}
