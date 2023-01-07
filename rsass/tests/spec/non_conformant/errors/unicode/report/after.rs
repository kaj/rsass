//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/unicode/report/after.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("after")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("foo{;öüäöüäöü"),
        "Error: expected \"{\".\
         \n  ,\
         \n1 | foo{;öüäöüäöü\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
    );
}
