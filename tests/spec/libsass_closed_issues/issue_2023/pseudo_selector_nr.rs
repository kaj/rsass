//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2023/pseudo-selector-nr.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "#4 {\r\
             \n    color: red;\r\
             \n}\r\
             \n"
        )
        .unwrap_err(),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | #4{\
         \n  |  ^\
         \n  \'\
         \n  input.scss 1:2  root stylesheet\
         \n",
    );
}
