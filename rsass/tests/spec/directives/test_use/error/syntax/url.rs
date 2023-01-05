//! Tests auto-converted from "sass-spec/spec/directives/use/error/syntax/url.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("url")
}

#[test]
#[ignore] // wrong error
fn empty() {
    assert_eq!(
        runner().err(
            "@use \"\";\n"
        ),
        "Error: The default namespace \"\" is not a valid Sass identifier.\n\
         \nRecommendation: add an \"as\" clause to define an explicit namespace.\
         \n  ,\
         \n1 | @use \"\";\
         \n  | ^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn non_identifier() {
    assert_eq!(
        runner().err(
            "@use \"123\";\n"
        ),
        "Error: The default namespace \"123\" is not a valid Sass identifier.\n\
         \nRecommendation: add an \"as\" clause to define an explicit namespace.\
         \n  ,\
         \n1 | @use \"123\";\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn unquoted() {
    assert_eq!(
        runner().err("@use foo;\n"),
        "Error: Expected string.\
         \n  ,\
         \n1 | @use foo;\
         \n  |      ^\
         \n  \'\
         \n  input.scss 1:6  root stylesheet",
    );
}
