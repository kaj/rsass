//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/return-in-root.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@return 42;"),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n1 | @return 42;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
