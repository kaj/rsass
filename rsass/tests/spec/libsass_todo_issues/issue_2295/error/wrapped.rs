//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2295/error/wrapped.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("wrapped")
        .mock_file("include.scss", "@if (true) {\r\n  display: none;\r\n}")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            ".my-scope {\r\
             \n  @import \'include.scss\';\r\
             \n}"
        ),
        "Error: expected \"{\".\
         \n  ,\
         \n2 |   display: none;\
         \n  |                ^\
         \n  \'\
         \n  include.scss 2:16  @import\
         \n  input.scss 2:11    root stylesheet",
    );
}
