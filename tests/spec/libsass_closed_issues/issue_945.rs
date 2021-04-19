//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_945.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            ".a {\
             \n  b: c;\
             \n  d:\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Expected expression.\
         \n  ,\
         \n3 |   d:\
         \n  |     ^\
         \n  \'\
         \n  input.scss 3:5  root stylesheet\
         \n",
    );
}
