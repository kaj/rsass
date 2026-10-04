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
        "Error: You may not @extend selectors across media queries.\
         \n    ,\
         \n1   | .foo {\
         \n    | ==== extended selector\
         \n... |\
         \n5   | @media print {\
         \n    |        ===== extension @media\
         \n... |\
         \n7   |     @extend .foo;\
         \n    |     ^^^^^^^^^^^^ extension\
         \n    \'\
         \n  input.scss 7:5  root stylesheet",
    );
}
