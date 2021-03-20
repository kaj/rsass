//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1251.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  yellow: yellow;\
            \n  red: red;\
            \n  blue: blue;\
            \n  white: white;\
            \n  black: black;\
            \n  eval: if(red, yellow, null);\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  yellow: yellow;\
        \n  red: red;\
        \n  blue: blue;\
        \n  white: white;\
        \n  black: black;\
        \n  eval: yellow;\
        \n}\
        \n"
    );
}
