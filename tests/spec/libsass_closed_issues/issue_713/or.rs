//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_713/or.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@function or() {\
             \n  @return \"or\";\
             \n}\
             \n\
             \ntest {\
             \n  or: or();\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function or() {\
         \n  | ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet\
         \n",
    );
}
