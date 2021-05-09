//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2295/error/basic.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().mock_file("include.scss", "display: none;\r\n")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            ".my-scope {\r\
             \n  @import \'include.scss\';\r\
             \n}"
        ),
        "Error: expected \"{\".\
         \n  ,\
         \n1 | display: none;\
         \n  |              ^\
         \n  \'\
         \n  include.scss 1:14  @import\
         \n  input.scss 2:11    root stylesheet",
    );
}
