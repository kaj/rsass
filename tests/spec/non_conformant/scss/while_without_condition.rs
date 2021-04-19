//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/while_without_condition.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@while {\
             \n\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Expected expression.\
         \n  ,\
         \n1 | @while {\
         \n  |        ^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet\
         \n",
    );
}
