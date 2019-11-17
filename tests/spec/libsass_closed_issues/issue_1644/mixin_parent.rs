//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1644/mixin-parent.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin parent {\
             \n  @content;\
             \n}\
             \n\
             \n@include parent() {\
             \n  body.immobile & {\
             \n    margin-bottom: 0;\
             \n  }\
             \n}\
             \n"
        ).unwrap_err(),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n6 |   body.immobile & {\
         \n  |   ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 6:3  @content\
         \n  input.scss 2:3  parent()\
         \n  input.scss 5:1  root stylesheet\
         \n",
    );
}
