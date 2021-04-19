//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-varargs/with-default.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass("@function test($param...:\"default\") {}").unwrap_err(),
        "Error: expected \")\".\
         \n  ,\
         \n1 | @function test($param...:\"default\") {}\
         \n  |                         ^\
         \n  \'\
         \n  input.scss 1:25  root stylesheet",
    );
}
