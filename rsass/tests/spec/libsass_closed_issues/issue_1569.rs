//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1569.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1569")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "$common-border: \"foo\";\
             \n.nihilo & {\
             \n  .dijitMenu {\
             \n    border: $common-border;\
             \n    .dijitMenuItem {\
             \n      color: getColor(\'text-dark-main\');\
             \n    }\
             \n  }\
             \n}\n"
        ),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n2 | .nihilo & {\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
