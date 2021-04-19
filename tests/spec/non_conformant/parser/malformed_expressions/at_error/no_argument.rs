//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/malformed_expressions/at-error/no-argument.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@error;\
             \n"
        )
        .unwrap_err(),
        "Error: Expected expression.\
         \n  ,\
         \n1 | @error;\
         \n  |       ^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
}
