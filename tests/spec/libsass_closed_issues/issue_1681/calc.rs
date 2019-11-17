//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1681/calc.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function calc() {\
             \n  @return null;\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function calc() {\
         \n  | ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet\
         \n",
    );
}
