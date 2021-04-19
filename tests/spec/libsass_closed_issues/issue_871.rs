//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_871.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            ".bar {\
             \n  @extend .foo;\
             \n  color: green;\
             \n}"
        )
        .unwrap_err(),
        "Error: The target selector was not found.\
         \nUse \"@extend .foo !optional\" to avoid this error.\
         \n  ,\
         \n2 |   @extend .foo;\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
