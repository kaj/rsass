//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1082.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@font-face {\
            \n  font-family: \'My Font\';\
            \n  font-style: normal;\
            \n  font-weight: 300;\
            \n  src: local(\'My Font\'), local(\'My-Font\'),\
            \n    /* from http://.... original source of .eot */\
            \n    url(\'my-font.eot?#iefix\') format(\'embedded-opentype\'),\
            \n    /* from http://.... original source of .woff */\
            \n    url(\'my-font.woff\') format(\'woff\'),\
            \n    /* from http://.... original source of .ttf */\
            \n    url(\'my-font.ttf\') format(\'truetype\'),\
            \n    /* from http://.... original source of .svg */\
            \n    url(\'my-font.svg#MyFont\') format(\'svg\');\
            \n}\
            \n"
        )
        .unwrap(),
        "@font-face {\
        \n  font-family: \"My Font\";\
        \n  font-style: normal;\
        \n  font-weight: 300;\
        \n  src: local(\"My Font\"), local(\"My-Font\"), url(\"my-font.eot?#iefix\") format(\"embedded-opentype\"), url(\"my-font.woff\") format(\"woff\"), url(\"my-font.ttf\") format(\"truetype\"), url(\"my-font.svg#MyFont\") format(\"svg\");\
        \n}\
        \n"
    );
}
