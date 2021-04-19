//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/return-in-root.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass("@return 42;").unwrap_err(),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n1 | @return 42;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
