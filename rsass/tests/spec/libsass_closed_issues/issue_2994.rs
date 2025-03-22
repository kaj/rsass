//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2994.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2994")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            ".one-screen-page {\
             \n\t@extend %context-dark;\
             \n}\n\
             \n%context-dark {\
             \n\t.button-secondary-outline {\
             \n\t\t&:hover,\
             \n\t\t&:focus,\
             \n\t\t&:active,\
             \n\t\t&:hover {\
             \n\t\t\tcolor: #fca;\
             \n\t\t}\
             \n\t}\
             \n}\n"
        ),
        ".one-screen-page .button-secondary-outline:hover, .one-screen-page .button-secondary-outline:focus, .one-screen-page .button-secondary-outline:active {\
         \n  color: #fca;\
         \n}\n"
    );
}
