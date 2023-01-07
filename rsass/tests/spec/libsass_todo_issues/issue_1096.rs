//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1096.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1096")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "// line-endings in this file must be CRLF\r\
             \n@import url(\"foo\\\r\
             \nbar\");\r\
             \n@import url(\"foo\r\
             \nbar\");\r\
             \n@import url(foo\r\
             \nbar);\r\n"
        ),
        "Error: Expected \".\
         \n  ,\
         \n4 | @import url(\"foo\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 4:17  root stylesheet",
    );
}
