//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2307.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "0//#{\
             \n{\
             \n"
        )
        .unwrap_err(),
        "Error: expected \"}\".\
         \n  ,\
         \n2 | {\
         \n  |  ^\
         \n  \'\
         \n  input.scss 2:2  root stylesheet\
         \n",
    );
}
