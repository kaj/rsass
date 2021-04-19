//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-function-1.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "// double comma in middle of arglist\
             \n@function double-comma-error($a,,$b) {\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: expected \")\".\
         \n  ,\
         \n2 | @function double-comma-error($a,,$b) {\
         \n  |                                 ^\
         \n  \'\
         \n  input.scss 2:33  root stylesheet",
    );
}
