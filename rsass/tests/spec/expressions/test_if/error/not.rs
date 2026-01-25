//! Tests auto-converted from "sass-spec/spec/expressions/if/error/not.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("not")
}

#[test]
#[ignore] // missing error
fn and() {
    assert_eq!(
        runner().err("a {b: if(not css(1) and css(2): c)}\n"),
        "Error: expected \":\".\
         \n  ,\
         \n1 | a {b: if(not css(1) and css(2): c)}\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn test_else() {
    assert_eq!(
        runner().err("a {b: if(not else: c)}\n"),
        "Error: expected \"(\".\
         \n  ,\
         \n1 | a {b: if(not else: c)}\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 1:18  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn empty() {
    assert_eq!(
        runner().err("a {b: if(not: c)}\n"),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: if(not: c)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn not() {
    assert_eq!(
        runner().err("a {b: if(not not css(): c)}\n"),
        "Error: expected \"(\".\
         \n  ,\
         \n1 | a {b: if(not not css(): c)}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn or() {
    assert_eq!(
        runner().err("a {b: if(not css(1) or css(2): c)}\n"),
        "Error: expected \":\".\
         \n  ,\
         \n1 | a {b: if(not css(1) or css(2): c)}\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
    );
}
