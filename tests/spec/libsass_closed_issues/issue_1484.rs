//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1484.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
             \n"
        )
        .unwrap_err(),
        "Error: expected \"}\".\
         \n  ,\
         \n1 | div {\
         \n  |      ^\
         \n  \'\
         \n  input.scss 1:6  root stylesheet\
         \n",
    );
}
