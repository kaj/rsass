//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2023/class-selector-id.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("class-selector-id")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            ".3c {\r\
             \n    color: red;\r\
             \n}\r\n"
        ),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | .3c {\
         \n  |  ^\
         \n  \'\
         \n  input.scss 1:2  root stylesheet",
    );
}
