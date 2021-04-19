//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_712.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
             \n  content: \'foo\';\
             \n}\
             \n\
             \n@media print {\
             \n  .bar {\
             \n    @extend .foo;\
             \n  }\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: From line 1, column 1 of input.scss: \
         \n  ,\
         \n1 | .foo {\
         \n  | ^^^^^\
         \n  \'\
         \nYou may not @extend selectors across media queries.\
         \n  ,\
         \n7 |     @extend .foo;\
         \n  |     ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 7:5  root stylesheet\
         \n",
    );
}
