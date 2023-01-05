//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_63.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_63")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin testComments {\r\
             \n\t/* crash */\r\
             \n\tp {\r\
             \n\t\twidth: 100px;\r\
             \n\t}\r\
             \n}\r\
             \n\r\
             \n@media screen and (orientation:landscape) {\r\
             \n\t@include testComments;\t\r\
             \n}\r\n"),
        "@media screen and (orientation: landscape) {\
         \n  /* crash */\
         \n  p {\
         \n    width: 100px;\
         \n  }\
         \n}\n"
    );
}
