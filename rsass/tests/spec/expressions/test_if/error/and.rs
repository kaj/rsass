//! Tests auto-converted from "sass-spec/spec/expressions/if/error/and.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("and")
}

#[test]
#[ignore] // missing error
fn test_else() {
    assert_eq!(
        runner().err("a {b: if(css() and else: c)}\n"),
        "Error: expected \"(\".\
         \n  ,\
         \n1 | a {b: if(css() and else: c)}\
         \n  |                        ^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn empty() {
    assert_eq!(
        runner().err("a {b: if(css(1) and: c)}\n"),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: if(css(1) and: c)}\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn not() {
    assert_eq!(
        runner().err("a {b: if(css(1) and not css(2): c)}\n"),
        "Error: expected \"(\".\
         \n  ,\
         \n1 | a {b: if(css(1) and not css(2): c)}\
         \n  |                        ^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn or() {
    assert_eq!(
        runner().err("a {b: if(css(1) and css(2) or css(3): c)}\n"),
        "Error: expected \":\".\
         \n  ,\
         \n1 | a {b: if(css(1) and css(2) or css(3): c)}\
         \n  |                            ^\
         \n  \'\
         \n  input.scss 1:28  root stylesheet",
    );
}
