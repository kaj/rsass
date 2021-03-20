//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_246.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$content-width: 960px;\r\
            \n\r\
            \n/* demo.css: */\r\
            \n.selector {\r\
            \n  padding: 0 calc(100%/2 - #{$content-width/2})\r\
            \n}\r\
            \n\r\
            \n\r\
            \n/* bin/sassc demo.scss */\r\
            \n.selector {\r\
            \n  padding: 0 calc(100%/2 - #{$content-width/2}); }"
        )
        .unwrap(),
        "/* demo.css: */\
        \n.selector {\
        \n  padding: 0 calc(100%/2 - 480px);\
        \n}\
        \n/* bin/sassc demo.scss */\
        \n.selector {\
        \n  padding: 0 calc(100%/2 - 480px);\
        \n}\
        \n"
    );
}
