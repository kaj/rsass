//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1873.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1873")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            ".a {\
             \n  display: block;\
             \n}\n\
             \n.b {\
             \n  @at-root (with: media) {\
             \n    @extend .a;\
             \n  }\
             \n}"
        ),
        "Error: @extend may only be used within style rules.\
         \n  ,\
         \n7 |     @extend .a;\
         \n  |     ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 7:5  root stylesheet",
    );
}
