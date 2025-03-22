//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1585.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1585")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@mixin bar() {\
             \n  @at-root { @content; }\
             \n}\n\
             \n.test {\
             \n  @include bar() {\
             \n    color: yellow;\
             \n    .nest2 {\
             \n      color: green;\
             \n    }\
             \n  }\
             \n}\n"
        ),
        "Error: Declarations may only be used within style rules.\
         \n  ,\
         \n7 |     color: yellow;\
         \n  |     ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 7:5   @content\
         \n  input.scss 2:14  bar()\
         \n  input.scss 6:3   root stylesheet",
    );
}
