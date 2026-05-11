//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1681/expression.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("expression")
}

#[test]
#[ignore] // wrong error
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
         \n  |           ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
    );
}
