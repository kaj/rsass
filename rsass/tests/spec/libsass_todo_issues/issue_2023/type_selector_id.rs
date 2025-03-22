//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2023/type-selector-id.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("type-selector-id")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "1a {\r\
             \n    color: red;\r\
             \n}\r\n"
        ),
        "Error: expected selector.\
         \n  ,\
         \n1 | 1a {\
         \n  | ^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
