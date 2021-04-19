//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2295/error/wrapped.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            ".my-scope {\r\
             \n  @import \'include.scss\';\r\
             \n}"
        )
        .unwrap_err(),
        "Error: expected \"{\".\
         \n  ,\
         \n2 |   display: none;\
         \n  |                ^\
         \n  \'\
         \n  include.scss 2:16  @import\
         \n  input.scss 2:11    root stylesheet\
         \n",
    );
}
