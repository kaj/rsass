//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_713/and.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@function and() {\
             \n  @return \"and\";\
             \n}\
             \n\
             \ntest {\
             \n  and: and();\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function and() {\
         \n  | ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet\
         \n",
    );
}
