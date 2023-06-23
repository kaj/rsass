//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1670.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1670")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            ".this-should-error {\
             \n  @extend %an-undefined-placeholder;\
             \n}\n"
        ),
        "Error: The target selector was not found.\
         \nUse \"@extend %an-undefined-placeholder !optional\" to avoid this error.\
         \n  ,\
         \n2 |   @extend %an-undefined-placeholder;\
         \n  |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
