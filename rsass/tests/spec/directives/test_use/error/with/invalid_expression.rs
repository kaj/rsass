//! Tests auto-converted from "sass-spec/spec/directives/use/error/with/invalid_expression.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("invalid_expression")
        .mock_file("error/_other.scss", "$a: c !default;\n")
        .mock_file(
            "module_loaded_later/_configured.scss",
            "$a: c !default;\n",
        )
        .mock_file("module_loaded_later/_other.scss", "$b: d;\n")
        .mock_file("variable_defined_later/_other.scss", "$a: d !default;\n")
}

#[test]
#[ignore] // missing error
fn error() {
    let runner = runner().with_cwd("error");
    assert_eq!(
        runner.err("@use \"other\" with ($a: 1px + 1em);\n"),
        "Error: 1px and 1em have incompatible units.\
         \n  ,\
         \n1 | @use \"other\" with ($a: 1px + 1em);\
         \n  |                        ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
    );
}
#[test]
fn module_loaded_later() {
    let runner = runner().with_cwd("module_loaded_later");
    assert_eq!(
        runner.err(
            "@use \"configured\" with ($a: other.$b);\
             \n@use \"other\";\n"
        ),
        "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n1 | @use \"configured\" with ($a: other.$b);\
         \n  |                             ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:29  root stylesheet",
    );
}
#[test]
fn variable_defined_later() {
    let runner = runner().with_cwd("variable_defined_later");
    assert_eq!(
        runner.err(
            "@use \"other\" with ($a: $b);\
             \n$b: c;\n"
        ),
        "Error: Undefined variable.\
         \n  ,\
         \n1 | @use \"other\" with ($a: $b);\
         \n  |                        ^^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
    );
}
