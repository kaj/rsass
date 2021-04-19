//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2143.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass("$map:;").unwrap_err(),
        "Error: Expected expression.\
         \n  ,\
         \n1 | $map:;\
         \n  |      ^\
         \n  \'\
         \n  input.scss 1:6  root stylesheet",
    );
}
