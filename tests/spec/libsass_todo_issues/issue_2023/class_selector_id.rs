//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2023/class-selector-id.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            ".3c {\r\
             \n    color: red;\r\
             \n}\r\
             \n"
        )
        .unwrap_err(),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | .3c{\
         \n  |  ^\
         \n  \'\
         \n  input.scss 1:2  root stylesheet",
    );
}
