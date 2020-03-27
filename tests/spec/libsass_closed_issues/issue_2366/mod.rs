//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2366"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_2366/global.hrx"
#[test]
#[ignore] // wrong result
fn global() {
    assert_eq!(
        rsass(
            ".item {\r\
            \n    display: inline-block;\r\
            \n\r\
            \n    :global(> .ext-link) {\r\
            \n        background: #000;\r\
            \n    }\r\
            \n}\r\
            \n\r\
            \n.link {\r\
            \n    color: red;\r\
            \n}\r\
            \n\r\
            \n.textLink {\r\
            \n    @extend .link;\r\
            \n    padding: 0 10px;\r\
            \n}"
        )
        .unwrap(),
        ".item {\
        \n  display: inline-block;\
        \n}\
        \n.item :global(> .ext-link) {\
        \n  background: #000;\
        \n}\
        \n.link, .textLink {\
        \n  color: red;\
        \n}\
        \n.textLink {\
        \n  padding: 0 10px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2366/has.hrx"
#[test]
#[ignore] // wrong result
fn has() {
    assert_eq!(
        rsass(
            ".item {\r\
            \n    display: inline-block;\r\
            \n\r\
            \n    :has(> .ext-link) {\r\
            \n        background: #000;\r\
            \n    }\r\
            \n}\r\
            \n\r\
            \n.link {\r\
            \n    color: red;\r\
            \n}\r\
            \n\r\
            \n.textLink {\r\
            \n    @extend .link;\r\
            \n    padding: 0 10px;\r\
            \n}"
        )
        .unwrap(),
        ".item {\
        \n  display: inline-block;\
        \n}\
        \n.item :has(> .ext-link) {\
        \n  background: #000;\
        \n}\
        \n.link, .textLink {\
        \n  color: red;\
        \n}\
        \n.textLink {\
        \n  padding: 0 10px;\
        \n}\
        \n"
    );
}
