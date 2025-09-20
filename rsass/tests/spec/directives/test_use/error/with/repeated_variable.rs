//! Tests auto-converted from "sass-spec/spec/directives/use/error/with/repeated_variable.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("repeated_variable")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@use \"other\" with ($a: b, $a: c);\n"),
        "Error: The same variable may only be configured once.\
         \n  ,\
         \n1 | @use \"other\" with ($a: b, $a: c);\
         \n  |                           ^^^^^\
         \n  \'\
         \n  input.scss 1:27  root stylesheet",
    );
}
