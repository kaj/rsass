//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1651/with.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("with")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "a {\
             \n  display: block;\
             \n}\
             \n.b {\
             \n  @at-root (with: media) {\
             \n    @extend .a;\
             \n  }\
             \n} \n"
        ),
        "Error: @extend may only be used within style rules.\
         \n  ,\
         \n6 |     @extend .a;\
         \n  |     ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 6:5  root stylesheet",
    );
}
