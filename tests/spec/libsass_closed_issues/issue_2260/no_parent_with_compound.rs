//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2260/no-parent-with-compound.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin test() {\
             \n  @at-root {\
             \n    @content;\
             \n  }\
             \n}\
             \n\
             \n@include test {\
             \n  .test & {\
             \n    property: value;\
             \n   }\
             \n}\
             \n"
        ).unwrap_err(),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n8 |   .test & {\
         \n  |   ^^^^^^^^\
         \n  \'\
         \n  input.scss 8:3  @content\
         \n  input.scss 3:5  test()\
         \n  input.scss 7:1  root stylesheet",
    );
}
