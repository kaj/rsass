//! Tests auto-converted from "sass-spec/spec/directives/use/error/syntax/with.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("with")
}

#[test]
#[ignore] // wrong error
fn before_as() {
    assert_eq!(
        runner().err("@use \"other\" with ($a: b) as c;\n\n"),
        "Error: expected \";\".\
         \n  ,\
         \n1 | @use \"other\" with ($a: b) as c;\
         \n  |                           ^\
         \n  \'\
         \n  input.scss 1:27  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn default() {
    assert_eq!(
        runner().err("@use \"other\" with ($a: b !default);\n"),
        "Error: expected \")\".\
         \n  ,\
         \n1 | @use \"other\" with ($a: b !default);\
         \n  |                          ^\
         \n  \'\
         \n  input.scss 1:26  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn empty() {
    assert_eq!(
        runner().err("@use \"other\" with ();\n"),
        "Error: expected \"$\".\
         \n  ,\
         \n1 | @use \"other\" with ();\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn extra_comma() {
    assert_eq!(
        runner().err("@use \"other\" with ($a: b,,);\n"),
        "Error: expected \")\".\
         \n  ,\
         \n1 | @use \"other\" with ($a: b,,);\
         \n  |                          ^\
         \n  \'\
         \n  input.scss 1:26  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn missing_keyword() {
    assert_eq!(
        runner().err("@use \"other\" with (a);\n"),
        "Error: expected \"$\".\
         \n  ,\
         \n1 | @use \"other\" with (a);\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn missing_value() {
    assert_eq!(
        runner().err("@use \"other\" with ($a);\n"),
        "Error: expected \":\".\
         \n  ,\
         \n1 | @use \"other\" with ($a);\
         \n  |                      ^\
         \n  \'\
         \n  input.scss 1:22  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn namespace_variable() {
    assert_eq!(
        runner().err("@use \"other\" with (a.$b: c);\n"),
        "Error: expected \"$\".\
         \n  ,\
         \n1 | @use \"other\" with (a.$b: c);\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn no_arguments() {
    assert_eq!(
        runner().err("@use \"other\" with;\n"),
        "Error: expected \"(\".\
         \n  ,\
         \n1 | @use \"other\" with;\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 1:18  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn space_after_dollar() {
    assert_eq!(
        runner().err("@use \"other\" with ($ a: b);\n"),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | @use \"other\" with ($ a: b);\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
    );
}
