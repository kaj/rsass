//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/known_incompatible/minus.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().err("a {b: calc(1px - 1s)}\n"),
        "Error: 1px and 1s are incompatible.\
         \n  ,\
         \n1 | a {b: calc(1px - 1s)}\
         \n  |            ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
