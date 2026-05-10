//! Tests auto-converted from "sass-spec/spec/expressions/if/error/missing.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("missing")
}

#[test]
#[ignore] // missing error
fn condition() {
    assert_eq!(
        runner().err("a {b: if(c)}\n"),
        "DEPRECATION WARNING [if-function]: The Sass if() syntax is deprecated in favor of the modern CSS syntax.\n\
         \nMore info: https://sass-lang.com/d/if-function\n\
         \n  ,\
         \n1 | a {b: if(c)}\
         \n  |       ^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: Missing argument $if-true.\
         \n  ,--> input.scss\
         \n1 | a {b: if(c)}\
         \n  |       ^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function if($condition, $if-true, $if-false) {\
         \n  |           =================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
