//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_246.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_246")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$content-width: 960px;\r\
             \n\r\
             \n/* demo.css: */\r\
             \n.selector {\r\
             \n  padding: 0 calc(100%/2 - #{$content-width/2})\r\
             \n}\r\
             \n\r\
             \n\r\
             \n/* bin/sassc demo.scss */\r\
             \n.selector {\r\
             \n  padding: 0 calc(100%/2 - #{$content-width/2}); }"),
        "/* demo.css: */\
         \n.selector {\
         \n  padding: 0 calc(50% - 480px);\
         \n}\
         \n/* bin/sassc demo.scss */\
         \n.selector {\
         \n  padding: 0 calc(50% - 480px);\
         \n}\n"
    );
}
