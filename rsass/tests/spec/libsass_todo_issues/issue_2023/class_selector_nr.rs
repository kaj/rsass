//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2023/class-selector-nr.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("class-selector-nr")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            ".3 {\r\
             \n    color: red;\r\
             \n}\r\n"
        ),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | .3 {\
         \n  |  ^\
         \n  \'\
         \n  input.scss 1:2  root stylesheet",
    );
}
