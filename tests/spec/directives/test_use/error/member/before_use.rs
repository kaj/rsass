//! Tests auto-converted from "sass-spec/spec/directives/use/error/member/before_use.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("before_use")
        .mock_file(
            "function/other.scss",
            "@function member() {@return null}\n",
        )
        .mock_file("variable_declaration/other.scss", "$member: value;\n")
        .mock_file(
            "variable_declaration_without_namespace/other.scss",
            "$member: from other;\n",
        )
        .mock_file("variable_use/other.scss", "$member: value;\n")
}

#[test]
fn function() {
    let runner = runner().with_cwd("function");
    assert_eq!(
        runner.err(
            "$variable: other.member();\
             \n@use \"other\";\n"
        ),
        "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n1 | $variable: other.member();\
         \n  |            ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn variable_declaration() {
    let runner = runner().with_cwd("variable_declaration");
    assert_eq!(
        runner.err(
            "other.$member: value;\
             \n@use \"other\";\n"
        ),
        "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n1 | other.$member: value;\
         \n  | ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn variable_declaration_without_namespace() {
    let runner = runner().with_cwd("variable_declaration_without_namespace");
    assert_eq!(
        runner.err(
            "$member: from input;\n\
             \n@use \"other\" as *;\n\
             \na {b: $member}\n"
        ),
        "Error: This module and the new module both define a variable named \"$member\".\
         \n  ,\
         \n3 | @use \"other\" as *;\
         \n  | ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
}
#[test]
fn variable_use() {
    let runner = runner().with_cwd("variable_use");
    assert_eq!(
        runner.err(
            "$variable: other.$member;\
             \n@use \"other\";\n"
        ),
        "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n1 | $variable: other.$member;\
         \n  |            ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
