//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/malformed_expressions/at-warn/no-argument.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@warn;\
             \n"
        )
        .unwrap_err(),
        "Error: Expected expression.\
         \n  ,\
         \n1 | @warn;\
         \n  |      ^\
         \n  \'\
         \n  input.scss 1:6  root stylesheet\
         \n",
    );
}
