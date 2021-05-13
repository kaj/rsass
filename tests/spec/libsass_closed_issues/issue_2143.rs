//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2143.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("$map:;"),
        "Error: Expected expression.\
         \n  ,\
         \n1 | $map:;\
         \n  |      ^\
         \n  \'\
         \n  input.scss 1:6  root stylesheet",
    );
}
