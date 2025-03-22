//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/known_incompatible/unknown_and_none.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unknown_and_none")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err("a {b: calc(1unknown + 1)}\n"),
        "Error: 1unknown and 1 are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1unknown + 1)}\
         \n  |            ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
