//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_712.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_712")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            ".foo {\
             \n  content: \'foo\';\
             \n}\n\
             \n@media print {\
             \n  .bar {\
             \n    @extend .foo;\
             \n  }\
             \n}\n"
        ),
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
         \n  input.scss 7:5  root stylesheet",
    );
}
