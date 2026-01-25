//! Tests auto-converted from "sass-spec/spec/expressions/if/error/paren.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("paren")
}

#[test]
#[ignore] // missing error
fn test_else() {
    assert_eq!(
        runner().err("a {b: if((else): c)}\n"),
        "Error: expected \"(\".\
         \n  ,\
         \n1 | a {b: if((else): c)}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn empty() {
    assert_eq!(
        runner().err("a {b: if((): c)}\n"),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: if((): c)}\
         \n  |           ^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
    );
}
