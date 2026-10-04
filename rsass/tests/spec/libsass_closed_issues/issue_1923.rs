//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1923.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1923")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "%btnBase {\
             \n  display: none;\
             \n}\n\
             \n@mixin mobile {\
             \n  @media only screen and (min-width:300px) {\
             \n    @content;\
             \n  }\
             \n}\n\
             \n@include mobile {\
             \n  a {\
             \n    @extend %btnBase;\
             \n  }\
             \n}"
        ),
        "Error: You may not @extend selectors across media queries.\
         \n    ,\
         \n1   | %btnBase {\
         \n    | ======== extended selector\
         \n... |\
         \n6   |   @media only screen and (min-width:300px) {\
         \n    |          ================================= extension @media\
         \n... |\
         \n13  |     @extend %btnBase;\
         \n    |     ^^^^^^^^^^^^^^^^ extension\
         \n    \'\
         \n  input.scss 13:5  @content\
         \n  input.scss 7:5   mobile()\
         \n  input.scss 11:1  root stylesheet",
    );
}
