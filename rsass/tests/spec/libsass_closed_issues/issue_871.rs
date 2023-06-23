//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_871.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_871")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            ".bar {\
             \n  @extend .foo;\
             \n  color: green;\
             \n}"
        ),
        "Error: The target selector was not found.\
         \nUse \"@extend .foo !optional\" to avoid this error.\
         \n  ,\
         \n2 |   @extend .foo;\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
