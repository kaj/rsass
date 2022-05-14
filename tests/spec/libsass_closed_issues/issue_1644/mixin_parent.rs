//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1644/mixin-parent.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin-parent")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@mixin parent {\
             \n  @content;\
             \n}\n\
             \n@include parent() {\
             \n  body.immobile & {\
             \n    margin-bottom: 0;\
             \n  }\
             \n}\n"
        ),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n6 |   body.immobile & {\
         \n  |   ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 6:3  @content\
         \n  input.scss 2:3  parent()\
         \n  input.scss 5:1  root stylesheet",
    );
}
