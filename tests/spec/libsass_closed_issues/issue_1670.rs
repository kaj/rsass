//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1670.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            ".this-should-error {\
             \n  @extend %an-undefined-placeholder;\
             \n}\
             \n"
        ).unwrap_err(),
        "Error: The target selector was not found.\
         \nUse \"@extend %an-undefined-placeholder !optional\" to avoid this error.\
         \n  ,\
         \n2 |   @extend %an-undefined-placeholder;\
         \n  |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
