//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2023/id-selector-nr.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "#2 {\r\
             \n    color: red;\r\
             \n}\r\
             \n"
        )
        .unwrap_err(),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | #2{\
         \n  |  ^\
         \n  \'\
         \n  input.scss 1:2  root stylesheet\
         \n",
    );
}
