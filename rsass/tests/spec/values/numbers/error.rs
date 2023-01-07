//! Tests auto-converted from "sass-spec/spec/values/numbers/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

#[test]
#[ignore] // missing error
fn plus_dot() {
    assert_eq!(
        runner().err(
            "// Regression test for sass/dart-sass#1856\
             \na {b: +.}\n"
        ),
        "Error: Expected digit.\
         \n  ,\
         \n2 | a {b: +.}\
         \n  |         ^\
         \n  \'\
         \n  input.scss 2:9  root stylesheet",
    );
}
