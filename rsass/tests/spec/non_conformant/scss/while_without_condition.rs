//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/while_without_condition.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("while_without_condition")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@while {\n\
             \n}\n"
        ),
        "Error: Expected expression.\
         \n  ,\
         \n1 | @while {\
         \n  |        ^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet",
    );
}
