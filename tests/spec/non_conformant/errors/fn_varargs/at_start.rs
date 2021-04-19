//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-varargs/at-start.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass("@function test($rest...,$param) {}").unwrap_err(),
        "Error: expected \")\".\
         \n  ,\
         \n1 | @function test($rest...,$param) {}\
         \n  |                        ^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
    );
}
