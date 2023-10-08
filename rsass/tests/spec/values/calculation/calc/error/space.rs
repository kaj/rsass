//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/space.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("space")
}

#[test]
fn number_calc() {
    assert_eq!(
        runner().err("a {b: calc(1 calc(1px + 1%))}\n"),
        "Error: Missing math operator.\
         \n  ,\
         \n1 | a {b: calc(1 calc(1px + 1%))}\
         \n  |            ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn number_number() {
    assert_eq!(
        runner().err("a {b: calc(1 2)}\n"),
        "Error: Missing math operator.\
         \n  ,\
         \n1 | a {b: calc(1 2)}\
         \n  |            ^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn number_number_string() {
    assert_eq!(
        runner().err("a {b: calc(1 2 c)}\n"),
        "Error: Missing math operator.\
         \n  ,\
         \n1 | a {b: calc(1 2 c)}\
         \n  |            ^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn number_operation() {
    assert_eq!(
        runner().err("a {b: calc(1 3 + 4)}\n"),
        "Error: Missing math operator.\
         \n  ,\
         \n1 | a {b: calc(1 3 + 4)}\
         \n  |            ^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn number_paren() {
    assert_eq!(
        runner().err("a {b: calc(1 (3))}\n"),
        "Error: Missing math operator.\
         \n  ,\
         \n1 | a {b: calc(1 (3))}\
         \n  |            ^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn operation_operation() {
    assert_eq!(
        runner().err("a {b: calc(1 + 2 3 + 4)}\n"),
        "Error: Missing math operator.\
         \n  ,\
         \n1 | a {b: calc(1 + 2 3 + 4)}\
         \n  |            ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn string_number_number() {
    assert_eq!(
        runner().err("a {b: calc(c 1 2)}\n"),
        "Error: Missing math operator.\
         \n  ,\
         \n1 | a {b: calc(c 1 2)}\
         \n  |              ^^^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
    );
}
#[test]
fn through_variable() {
    assert_eq!(
        runner().err(
            "$c: 1;\
             \n$d: 2;\
             \na {b: calc($c $d)}\n"
        ),
        "Error: Missing math operator.\
         \n  ,\
         \n3 | a {b: calc($c $d)}\
         \n  |            ^^^^^\
         \n  \'\
         \n  input.scss 3:12  root stylesheet",
    );
}
