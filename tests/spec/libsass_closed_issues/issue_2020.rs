//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2020.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "form {\r\
            \n  $selector: nth(&, 1);\r\
            \n  sel: inspect($selector);\r\
            \n  $selector: nth($selector, 1);\r\
            \n  sel: inspect($selector);\r\
            \n} "
        )
        .unwrap(),
        "form {\
        \n  sel: form;\
        \n  sel: form;\
        \n}\
        \n"
    );
}
