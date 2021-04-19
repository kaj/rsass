//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/interpolation/error-1.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "\'#{)\'{\
             \n"
        )
        .unwrap_err(),
        "Error: Expected expression.\
         \n  ,\
         \n1 | \'#{)\'{\
         \n  |  ^^\
         \n  \'\
         \n  input.scss 1:2  root stylesheet\
         \n",
    );
}
