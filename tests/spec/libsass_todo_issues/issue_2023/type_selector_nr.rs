//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2023/type-selector-nr.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "1 {\r\
             \n    color: red;\r\
             \n}\r\
             \n"
        )
        .unwrap_err(),
        "Error: expected selector.\
         \n  ,\
         \n1 | 1{\
         \n  | ^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
