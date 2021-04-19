//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1096.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "// line-endings in this file must be CRLF\r\
             \n@import url(\"foo\\\r\
             \nbar\");\r\
             \n@import url(\"foo\r\
             \nbar\");\r\
             \n@import url(foo\r\
             \nbar);\r\
             \n"
        )
        .unwrap_err(),
        "Error: Expected \".\
         \n  ,\
         \n4 | @import url(\"foo\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 4:17  root stylesheet",
    );
}
