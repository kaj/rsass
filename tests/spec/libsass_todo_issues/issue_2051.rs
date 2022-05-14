//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_2051.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2051")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            ":global(.thing) {\
             \n    color: red;\
             \n}\n\
             \n:global(.thing[disabled]) {\
             \n    @extend .thing;\
             \n    background: blue;\
             \n}"
        ),
        "Error: The target selector was not found.\
         \nUse \"@extend .thing !optional\" to avoid this error.\
         \n  ,\
         \n6 |     @extend .thing;\
         \n  |     ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 6:5  root stylesheet",
    );
}
