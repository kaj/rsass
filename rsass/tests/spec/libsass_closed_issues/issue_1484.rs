//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1484.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1484")
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
