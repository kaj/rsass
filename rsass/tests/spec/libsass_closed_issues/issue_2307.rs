//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2307.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2307")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "0//#{\
             \n{\n"
        ),
        "Error: expected \"}\".\
         \n  ,\
         \n2 | {\
         \n  |  ^\
         \n  \'\
         \n  input.scss 2:2  root stylesheet",
    );
}
