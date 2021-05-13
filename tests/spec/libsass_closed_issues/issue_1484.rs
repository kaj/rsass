//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1484.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("div {\n"),
        "Error: expected \"}\".\
         \n  ,\
         \n1 | div {\
         \n  |      ^\
         \n  \'\
         \n  input.scss 1:6  root stylesheet",
    );
}
