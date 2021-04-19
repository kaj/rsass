//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/malformed_expressions/at-debug/incomplete-expression.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@debug(\"foo\";\
             \n"
        )
        .unwrap_err(),
        "Error: expected \")\".\
         \n  ,\
         \n1 | @debug(\"foo\";\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
