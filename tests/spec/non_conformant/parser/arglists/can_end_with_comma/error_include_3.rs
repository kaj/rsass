//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-include-3.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "// incorectly terminated\
             \n.error {\
             \n  @include incorrectly-terminated($a,$b,;\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: expected \")\".\
         \n  ,\
         \n3 |   @include incorrectly-terminated($a,$b,;\
         \n  |                                         ^\
         \n  \'\
         \n  input.scss 3:41  root stylesheet\
         \n",
    );
}
