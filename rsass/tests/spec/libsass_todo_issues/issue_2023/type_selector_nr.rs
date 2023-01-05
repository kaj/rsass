//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2023/type-selector-nr.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("type-selector-nr")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "1 {\r\
             \n    color: red;\r\
             \n}\r\n"
        ),
        "Error: expected selector.\
         \n  ,\
         \n1 | 1{\
         \n  | ^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
