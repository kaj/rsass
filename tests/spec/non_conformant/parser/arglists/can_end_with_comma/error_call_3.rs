//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-call-3.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "// incorectly terminated\
             \n.error {\
             \n  a: incorrectly-terminated($a,$b,;\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: expected \")\".\
         \n  ,\
         \n3 |   a: incorrectly-terminated($a,$b,;\
         \n  |                                   ^\
         \n  \'\
         \n  input.scss 3:35  root stylesheet\
         \n",
    );
}
