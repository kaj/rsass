//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/error-directive.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@error \"Buckle your seatbelt Dorothy, \'cause Kansas is going bye-bye\"\
             \n"
        ).unwrap_err(),
        "Error: \"Buckle your seatbelt Dorothy, \'cause Kansas is going bye-bye\"\
         \n  ,\
         \n1 | @error \"Buckle your seatbelt Dorothy, \'cause Kansas is going bye-bye\"\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet\
         \n",
    );
}
