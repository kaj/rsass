//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1681/expression.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("expression")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@function expression() {\
             \n  @return null;\
             \n}\n"
        ),
        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function expression() {\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
