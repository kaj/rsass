//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1681/calc.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("calc")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@function calc() {\
             \n  @return null;\
             \n}\n"
        ),
        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function calc() {\
         \n  | ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
