//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/error-directive.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error-directive")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@error \"Buckle your seatbelt Dorothy, \'cause Kansas is going bye-bye\"\n"
        ),
        "Error: \"Buckle your seatbelt Dorothy, \'cause Kansas is going bye-bye\"\
         \n  ,\
         \n1 | @error \"Buckle your seatbelt Dorothy, \'cause Kansas is going bye-bye\"\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
