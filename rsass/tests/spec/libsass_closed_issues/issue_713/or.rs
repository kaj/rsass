//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_713/or.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("or")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@function or() {\
             \n  @return \"or\";\
             \n}\n\
             \ntest {\
             \n  or: or();\
             \n}\n"
        ),
        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function or() {\
         \n  | ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
