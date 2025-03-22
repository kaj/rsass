//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/return-in-root.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("return-in-root")
}

#[test]
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
