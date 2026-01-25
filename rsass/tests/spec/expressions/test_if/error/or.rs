//! Tests auto-converted from "sass-spec/spec/expressions/if/error/or.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("or")
}

#[test]
#[ignore] // missing error
fn and() {
    assert_eq!(
        runner().err("a {b: if(css(1) or css(2) and css(3): c)}\n"),
        "Error: expected \":\".\
         \n  ,\
         \n1 | a {b: if(css(1) or css(2) and css(3): c)}\
         \n  |                           ^\
         \n  \'\
         \n  input.scss 1:27  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn test_else() {
    assert_eq!(
        runner().err("a {b: if(css(1) or else: c)}\n"),
        "Error: expected \"(\".\
         \n  ,\
         \n1 | a {b: if(css(1) or else: c)}\
         \n  |                        ^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn empty() {
    assert_eq!(
        runner().err("a {b: if(css(1) or: c)}\n"),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: if(css(1) or: c)}\
         \n  |                   ^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn not() {
    assert_eq!(
        runner().err("a {b: if(css(1) or not css(2): c)}\n"),
        "Error: expected \"(\".\
         \n  ,\
         \n1 | a {b: if(css(1) or not css(2): c)}\
         \n  |                       ^\
         \n  \'\
         \n  input.scss 1:23  root stylesheet",
    );
}
