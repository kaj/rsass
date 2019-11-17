//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1923.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "%btnBase {\
             \n  display: none;\
             \n}\
             \n\
             \n@mixin mobile {\
             \n  @media only screen and (min-width:300px) {\
             \n    @content;\
             \n  }\
             \n}\
             \n\
             \n@include mobile {\
             \n  a {\
             \n    @extend %btnBase;\
             \n  }\
             \n}"
        )
        .unwrap_err(),
        "Error: From line 1, column 1 of input.scss: \
         \n  ,\
         \n1 | %btnBase {\
         \n  | ^^^^^^^^^\
         \n  \'\
         \nYou may not @extend selectors across media queries.\
         \n   ,\
         \n13 |     @extend %btnBase;\
         \n   |     ^^^^^^^^^^^^^^^^\
         \n   \'\
         \n  input.scss 13:5  root stylesheet\
         \n",
    );
}
