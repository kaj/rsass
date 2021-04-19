//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1569.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "$common-border: \"foo\";\
             \n.nihilo & {\
             \n  .dijitMenu {\
             \n    border: $common-border;\
             \n    .dijitMenuItem {\
             \n      color: getColor(\'text-dark-main\');\
             \n    }\
             \n  }\
             \n}\
             \n"
        ).unwrap_err(),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n2 | .nihilo & {\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
    );
}
