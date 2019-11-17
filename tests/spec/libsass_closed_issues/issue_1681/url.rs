//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1681/url.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function url() {\
             \n  @return null;\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function url() {\
         \n  | ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet\
         \n",
    );
}
