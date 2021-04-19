//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-error/simple.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass("@error \"error\";").unwrap_err(),
        "Error: \"error\"\
         \n  ,\
         \n1 | @error \"error\";\
         \n  | ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
