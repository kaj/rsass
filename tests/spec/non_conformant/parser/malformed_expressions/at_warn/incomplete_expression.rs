//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/malformed_expressions/at-warn/incomplete-expression.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@warn(\"foo\";\
             \n"
        )
        .unwrap_err(),
        "Error: expected \")\".\
         \n  ,\
         \n1 | @warn(\"foo\";\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet\
         \n",
    );
}
