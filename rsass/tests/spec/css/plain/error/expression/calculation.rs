//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression/calculation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("calculation")
        .mock_file("interpolation/plain.css", "a {b: calc(#{1px})}\n")
        .mock_file("line_noise/plain.css", "a {b: calc(#%^&)}\n")
        .mock_file("namespaced_function/plain.css", "a {b: calc(c.d())}\n")
        .mock_file("variable/plain.css", "a {b: calc($var)}\n")
        .mock_file("wrong_args/plain.css", "a {b: calc()}\n")
}

#[test]
fn interpolation() {
    let runner = runner().with_cwd("interpolation");
    assert_eq!(
        runner.err("@use \"plain\";\n"),
        "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | a {b: calc(#{1px})}\
         \n  |            ^^^^^^\
         \n  \'\
         \n  plain.css 1:12  @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn line_noise() {
    let runner = runner().with_cwd("line_noise");
    assert_eq!(
        runner.err("@use \"plain\";\n"),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: calc(#%^&)}\
         \n  |             ^\
         \n  \'\
         \n  plain.css 1:13  @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
fn namespaced_function() {
    let runner = runner().with_cwd("namespaced_function");
    assert_eq!(
        runner.err("@use \"plain\";\n"),
        "Error: Module namespaces aren\'t allowed in plain CSS.\
         \n  ,\
         \n1 | a {b: calc(c.d())}\
         \n  |            ^^^^^\
         \n  \'\
         \n  plain.css 1:12  @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
fn variable() {
    let runner = runner().with_cwd("variable");
    assert_eq!(
        runner.err("@use \"plain\";\n"),
        "Error: Sass variables aren\'t allowed in plain CSS.\
         \n  ,\
         \n1 | a {b: calc($var)}\
         \n  |            ^^^^\
         \n  \'\
         \n  plain.css 1:12  @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
fn wrong_args() {
    let runner = runner().with_cwd("wrong_args");
    assert_eq!(
        runner.err("@use \"plain\";\n"),
        "Error: Missing argument.\
         \n  ,\
         \n1 | a {b: calc()}\
         \n  |       ^^^^^^\
         \n  \'\
         \n  plain.css 1:7   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
