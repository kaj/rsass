//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/unicode/report/before.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("before")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("öüäöüäöü{a:c"),
        "Error: expected \"}\".\
         \n  ,\
         \n1 | öüäöüäöü{a:c\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
